use super::*;
use std::{io::{Read,Write}, net::TcpListener};
use serde_json::json;

fn serve(responses: Vec<(u16,String,Duration)>) -> (String, std::thread::JoinHandle<Vec<String>>) {
    let listener=TcpListener::bind("127.0.0.1:0").unwrap();listener.set_nonblocking(true).unwrap();
    let url=format!("http://{}/catalog",listener.local_addr().unwrap());
    let thread=thread::spawn(move || {
        let mut requests=Vec::new();
        for (status,body,delay) in responses {
            let deadline=Instant::now()+Duration::from_secs(3);
            let mut stream=loop {
                match listener.accept() {
                    Ok((stream,_))=>break stream,
                    Err(e) if e.kind()==std::io::ErrorKind::WouldBlock=>{
                        if Instant::now()>=deadline { return requests; }
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(_)=>return requests,
                }
            };
            stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
            stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
            let mut data=Vec::new();let mut bytes=[0u8;2048];
            while !data.windows(4).any(|p| p==b"\r\n\r\n") {
                let n=stream.read(&mut bytes).unwrap_or(0);if n==0 { break; }data.extend_from_slice(&bytes[..n]);
                if data.len()>32768 { break; }
            }
            requests.push(String::from_utf8_lossy(&data).lines().next().unwrap_or_default().to_owned());
            thread::sleep(delay);
            let response=format!("HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len());
            let _=stream.write_all(response.as_bytes());
        }
        requests
    });
    (url,thread)
}

fn network(url: &str) -> (Policy,FireworksNetwork) {
    let policy=super::super::tests::policy(None);
    let mut network=FireworksNetwork::new(&policy,"synthetic-credential-marker").unwrap();
    network.catalog_url=url.to_owned();network.base_url=url.trim_end_matches("/catalog").to_owned();
    network.backoff=Duration::from_millis(1);network.request_timeout=Duration::from_millis(100);
    (policy,network)
}

#[test]
fn pagination_preserves_query_and_escapes_opaque_tokens() {
    let (url,server)=serve(vec![
        (200,json!({"models":[],"nextPageToken":"next +%"}).to_string(),Duration::ZERO),
        (200,json!({"models":[]}).to_string(),Duration::ZERO),
    ]);
    let (p,mut n)=network(&url);assert!(n.catalog(&p,1_800_000_000).unwrap().is_empty());
    let calls=server.join().unwrap();assert_eq!(calls.len(),2);
    assert!(calls[0].contains("pageSize=100"));assert!(!calls[0].contains("pageToken="));
    assert!(calls[1].contains("pageSize=100"));assert!(calls[1].contains("pageToken=next+%2B%25"));
}

#[test]
fn repeated_page_token_is_rejected_not_looped() {
    let body=json!({"models":[],"nextPageToken":"same"}).to_string();
    let (url,server)=serve(vec![(200,body.clone(),Duration::ZERO),(200,body,Duration::ZERO)]);
    let (p,mut n)=network(&url);assert!(matches!(n.catalog(&p,1_800_000_000),Err(SelectionError::CatalogInvalid)));
    assert_eq!(server.join().unwrap().len(),2);
}

#[test]
fn page_limit_does_not_publish_partial_catalog() {
    let dir=tempfile::tempdir().unwrap();let file=dir.path().join("bot.toml");
    std::fs::write(&file,include_str!("../../../../config/bot.toml").replacen("max_pages = 100","max_pages = 1",1)).unwrap();
    let p=Policy::new(crate::bot_config::BotConfig::load(&file).unwrap(),None,"synthetic-credential-marker").unwrap();
    let (url,server)=serve(vec![(200,json!({"models":[],"nextPageToken":"another"}).to_string(),Duration::ZERO)]);
    let (_,mut n)=network(&url);assert!(matches!(n.catalog(&p,1_800_000_000),Err(SelectionError::CatalogInvalid)));
    assert_eq!(server.join().unwrap().len(),1);
}

#[test]
fn authorization_failures_are_not_retried_and_do_not_echo_body() {
    for (code,expected) in [(401,SelectionError::Unauthorized),(403,SelectionError::Forbidden)] {
        let (url,server)=serve(vec![(code,"sentinel-private-fixture".to_owned(),Duration::ZERO)]);
        let (p,mut n)=network(&url);let error=n.catalog(&p,1_800_000_000).unwrap_err();
        assert_eq!(error,expected);assert!(!format!("{error} {error:?}").contains("sentinel-private-fixture"));
        assert_eq!(server.join().unwrap().len(),1);
    }
}

#[test]
fn rate_limits_and_server_failures_have_bounded_retries() {
    for (code,expected) in [(429,SelectionError::RateLimited),(500,SelectionError::ProviderUnavailable),(503,SelectionError::ProviderUnavailable)] {
        let (url,server)=serve(vec![(code,"private response".to_owned(),Duration::ZERO);3]);
        let (p,mut n)=network(&url);assert_eq!(n.catalog(&p,1_800_000_000).unwrap_err(),expected);
        assert_eq!(server.join().unwrap().len(),3);
    }
}

#[test]
fn transient_failure_can_recover_without_changing_endpoint() {
    let (url,server)=serve(vec![(503,"ignored".to_owned(),Duration::ZERO),(200,json!({"models":[]}).to_string(),Duration::ZERO)]);
    let (p,mut n)=network(&url);assert!(n.catalog(&p,1_800_000_000).is_ok());
    let calls=server.join().unwrap();assert_eq!(calls.len(),2);assert_eq!(calls[0],calls[1]);
}

#[test]
fn timeout_is_bounded_and_typed() {
    let (url,server)=serve(vec![(200,json!({"models":[]}).to_string(),Duration::from_millis(200))]);
    let (p,mut n)=network(&url);n.attempts=1;n.request_timeout=Duration::from_millis(30);
    assert_eq!(n.catalog(&p,1_800_000_000).unwrap_err(),SelectionError::Timeout);
    assert_eq!(server.join().unwrap().len(),1);
}

#[test]
fn html_200_and_redirects_are_not_successful_catalogs() {
    for (status,body) in [(200,"<html>sentinel-private-fixture</html>"),(302,"redirect")] {
        let (url,server)=serve(vec![(status,body.to_owned(),Duration::ZERO)]);
        let (p,mut n)=network(&url);let error=n.catalog(&p,1_800_000_000).unwrap_err();
        assert!(!format!("{error} {error:?}").contains("sentinel-private-fixture"));assert_eq!(server.join().unwrap().len(),1);
    }
}
