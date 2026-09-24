type == "object"
and (keys == ["codeql", "python", "rust", "rust-security", "source-security", "workflow-policy"])
and all(.[]; .result == "success")
