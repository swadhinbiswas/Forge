with open("forge/app.py", "r") as f:
    content = f.read()

content = content.replace("proxy.load_url(self._dev_server_url)", 'proxy.load_url("main", self._dev_server_url)')

with open("forge/app.py", "w") as f:
    f.write(content)

with open("tests/test_router.py", "r") as f:
    content = f.read()

content = content.replace('app.bridge.register_command.assert_any_call("math:add", router.commands["math:add"])', 'app.bridge.register_command.assert_any_call("math:add", router.commands["math:add"], capability=None, version="1.0", internal=False)')
content = content.replace('app.bridge.register_command.assert_any_call("math:subtract", router.commands["math:subtract"])', 'app.bridge.register_command.assert_any_call("math:subtract", router.commands["math:subtract"], capability=None, version="1.0", internal=False)')

with open("tests/test_router.py", "w") as f:
    f.write(content)

