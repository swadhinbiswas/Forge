from forge import ForgeApp
app = ForgeApp()

@app.events.on("ready")
def setup(e=None):
    pass

# We will test fetching an arbitrary file like /etc/passwd or a local script fr
om JS                                                                           if __name__ == "__main__":
    app.run(debug=True)
