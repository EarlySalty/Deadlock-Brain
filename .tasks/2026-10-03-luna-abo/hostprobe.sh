#!/usr/bin/env bash

# Nur dieser vollständige Metadatenaufruf ist von der Compilerprobe ausgenommen.
brain_metadata_arguments_allowed() {
    [[ $# == 7 && ${1##*/} == cargo && $2 == metadata &&
       $3 == --format-version && $4 == 1 && $5 == --no-deps &&
       $6 == --manifest-path && $7 == /* && ${7##*/} == Cargo.toml ]]
}

brain_process_identity() {
    local pid=$1 line rest
    local -a fields=()
    IFS= read -r line 2>/dev/null < "/proc/$pid/stat" || return 1
    [[ $line == "$pid ("* && $line == *') '* ]] || return 1
    rest=${line##*) }
    read -r -a fields <<< "$rest"
    [[ ${#fields[@]} -ge 20 ]] || return 1
    printf '%s' "${fields[19]}"
}

brain_cargo_metadata_process_allowed() {
    local pid=$1 before after comm index
    local -a arguments=() repeated=()
    before=$(brain_process_identity "$pid") || return 1
    mapfile -d '' -t arguments 2>/dev/null < "/proc/$pid/cmdline" || return 1
    brain_metadata_arguments_allowed "${arguments[@]}" || return 1
    mapfile -d '' -t repeated 2>/dev/null < "/proc/$pid/cmdline" || return 1
    [[ ${#arguments[@]} == ${#repeated[@]} ]] || return 1
    for index in "${!arguments[@]}"; do
        [[ ${arguments[index]} == "${repeated[index]}" ]] || return 1
    done
    IFS= read -r comm 2>/dev/null < "/proc/$pid/comm" || return 1
    [[ $comm == cargo ]] || return 1
    after=$(brain_process_identity "$pid") || return 1
    [[ $before == "$after" ]]
}

# Die Ausgabe enthält ausschließlich Prozessmetadaten und die Klassifikation.
brain_hostprobe() {
    local snapshot pid ppid state comm busy=0
    snapshot=$(ps -eo pid=,ppid=,stat=,comm=) || return 75
    while read -r pid ppid state comm; do
        [[ $state != Z* ]] || continue
        case "$comm" in
            cargo)
                if brain_cargo_metadata_process_allowed "$pid"; then
                    printf '%s %s %s METADATA\n' "$pid" "$ppid" "$comm"
                else
                    printf '%s %s %s BLOCK\n' "$pid" "$ppid" "$comm"
                    busy=1
                fi
                ;;
            rustc|clippy-driver|rustdoc)
                printf '%s %s %s BLOCK\n' "$pid" "$ppid" "$comm"
                busy=1
                ;;
        esac
    done <<< "$snapshot"
    [[ $busy == 0 ]] || return 75
}

# Aufruf ausschließlich nach blockierendem Erwerb beider bestehenden Hostlocks.
brain_wait_for_compilers() {
    until brain_hostprobe; do
        sleep 30
    done
}
