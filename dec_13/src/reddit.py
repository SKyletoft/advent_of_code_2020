def day13_2():
    with open("src/input.txt", "r") as file:
        data = file.readlines()
 
    buses = [int(i) if i != "x" else 0 for i in data[1].split(",")]
 
    modulus = 1
    for b in buses:
        if b != 0:
            modulus *= b
 
    timestamp = 0
    for b in buses:
        if b != 0:
            timestamp += -buses.index(b) * (modulus // b) * pow(modulus // b, -1, b)
 
    return timestamp % modulus

def main():
	print(day13_2())

if __name__ == "__main__":
    main()