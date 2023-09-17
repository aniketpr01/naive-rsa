# Naive RSA

A naive bare-bones implementation of RSA algorithm

## Private and Public key generation

 1. Select two large prime numbers p and q - ***Private***
 2. Calculate N = P * Q - ***Public***
 3. Calculate Euler's phi(N)(co-prime of N) = φ(N) = (P-1)*(Q-1) - ***Private***
 4. Find e that has no common factor/co-prime with φ, using Basic Euclidean algorithm GCD(E, φ(N)) = 1) and 1 < E< φ(N) - ***Public***
 5. Choose d such that value d needs to hold the following condition: D∗E mod φ(N) = 1 or D*E = 1 mod φ(N) , since e and φ(N) are known, D is multiplicative inverse of e and can be found using extended euclidean algorithm - ***Private***

 • Public key is = (E, N)
 • Private key is = (D, N)

## Encryption and Decryption

 • Given a message M and a ciphered message C

### Encryption

 • Given the message M, the encrypted cipher-message is C = Message^e (mod N)

### Decryption

 • Given the cipher-message C, we can use M = C^d (mod N) to calculate the original message M.

### FINDING BIG PRIME NUMBERS

'96543390677764721740735128239245428995208381696823499601989570975501893576667 is prime' is a true statement!

RSA PUBLIC KEY ENCRYPTION

Plaintext:            'Coming tomorrow!'

Generating key pair...

* Private key is: d=0x0ae6893332356966ecb6eec38e11a8dc6f6ec7925f5d4f18eb3b1c6c400d39ab, n=0x1059cdcccb501e1a63126625551a7d4b28dd4077e5b850ee69a8551081d2c6f5,
* Public key is:  e=0x03, n=0x1059cdcccb501e1a63126625551a7d4b28dd4077e5b850ee69a8551081d2c6f5, key_size=256

Ciphertext:           '0x01a93489f8f4bfb95ed88a99ff4faa9894c3f2fa26f401df6f7e40601444fdb2'

Decrypted ciphertext: 'Coming tomorrow!'

### -----------------------------------

* Choose big primary numbers
* use miller-rabin for primality test
* use square and multiply for encryption and decryption
* choose e 65537 -> fermat's representation
* consider low hamming weight

## Miller-Rabin

* A probabilistic algorithm to determine if n is prime or not.
* For any prime p, a^p-1 = 1 mod p for 1 < a < p. This is Fermat's
Little Theorem.
* Input n and k are provided where n is greater than 2 and k is number of rounds of testing which determines the accuracy of test, more the rounds, more the accuracy
* Output: "composite" if n is composite, otherwise "probably prime"
