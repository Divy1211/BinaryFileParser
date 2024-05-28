import os


def main():
    for dir_, _, files in os.walk("./scxs/genie-rs"):
        for file in files:
            print(file)
            with open(os.path.join(dir_, file), "rb") as scx:
                ver = scx.read(4)
                print(ver)


if __name__ == "__main__":
    main()
