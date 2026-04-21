from forge import ForgeApp

app = ForgeApp()

@app.command("get_massive_data")
def get_massive_data():
    print("Python is generating massive array...")
    return b'\x00\x01\x02\x03\x04\x05' * 5_000_000

if __name__ == "__main__":
    app.run(debug=True)
