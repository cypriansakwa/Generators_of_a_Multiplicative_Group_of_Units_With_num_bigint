## Overview
This Rust program determines the generators of the multiplicative group $Z_n^\*$, which consists of all integers  modulo $n$ that are coprime with $n$ under multiplication. The code determines these generators by applying Euler's Totient function and prime factorization. The program uses the num-bigint, num-integer, and num-traits crates to handle arbitrary precision integers and mathematical operations. The multiplicative group $Z_n^\*$ is formed by the integers $\\{1,2,3,\cdots,n-1\\}$ that are coprime with $n$. This program calculates these generators by:
 - Computing Euler's Totient function $\phi(n)$.
 - Determining the prime factors of $\phi(n)$.
 - Identifying the numbers that serve as generators for the group.
## How It Works
- Euler's Totient Function $\phi(n)$: This function gives the number of integers up to $n$ that are coprime to $n$. 
- Prime Factorization: The code finds the prime factors of $\phi(n)$ to assist in determining if a candidate number is a generator.
- Check for Generator: For each candidate number, the code checks if it is capable of generating all elements of the group $Z_n^\*$ through exponentiation.
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- You can change the values of $n$ in the main function to test different cases.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
git clone https://github.com/cypriansakwa/Generators_of_a_Multiplicative_Group_of_Units_With_num_bigint.git
cd Generators_of_a_Multiplicative_Group_of_Units_With_num_bigint
