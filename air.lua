wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = "{\"query\":\"{\\n  getMenuBy(name: \\\"omelette\\\") {\\n    name {\\n      th\\n      en\\n    }\\n    price\\n  }\\n}\\n\",\"variables\":{},\"operationName\":\"\"}"