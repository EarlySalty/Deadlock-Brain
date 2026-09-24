(.version == "2.1.0") and (.runs | length > 0)
and all(.runs[];
  (.tool.driver.name == "CodeQL")
  and (.tool.driver.rules | length > 0)
  and all(.invocations[]?; .executionSuccessful != false)
  and all(.invocations[]?.toolExecutionNotifications[]?; .level != "error")
  and all(.results[]?; (.kind // "fail") != "fail" or (.level // "warning") == "none")
)
