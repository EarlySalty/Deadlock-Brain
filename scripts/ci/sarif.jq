# CodeQL query packs carry rules in tool.extensions, not necessarily in driver.rules.
(.version == "2.1.0") and (.runs | type == "array" and length > 0)
and all(.runs[];
  (.tool.driver.name == "CodeQL")
  and ([.tool.driver.rules[]?, .tool.extensions[]?.rules[]?]
       | any(.[]; (.properties.tags // [] | index("security")) != null))
  and (.invocations | type == "array" and length > 0)
  and all(.invocations[]; .executionSuccessful == true)
  and all(.invocations[].toolExecutionNotifications[]?; .level != "error")
  and all(.properties.metricResults[]?;
      if (.ruleId | endswith("number-of-files-extracted-with-errors"))
         or (.message.text == "Extraction errors")
         or (.message.text == "Files extracted - with errors")
      then .value == 0 else true end)
  and (.results | type == "array")
  and all(.results[]; (.kind // "fail") != "fail" or (.level // "warning") == "none")
)
