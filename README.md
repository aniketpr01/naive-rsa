# Naive RSA

A naive bare-bones implementation of RSA algorithm

## Implemented

* [x] Generate big numbers of 512 bits and 1024 bits.
* [x] Implement modular exponentiation operation needed by Rabin-Miller algorithm. (TESTED AND WORKING)
* [x] Implement modular exponentiation operation needed by Rabin-Miller algorithm. (TESTED AND WORKING)
* [x] Implement Rabin Miller algorithm. (TESTED AND WORKING)
* [x] Optimize Rabin-Miller algorithm by using choosing [2,13,23,1662803]
* [x] Implement Extended Euclides Algorithm to search for mcd of two suposed prime numbers.
* [x] Encrypt messages with Public Key.
* [x] Decrypt messages with Secret Key. (TESTED AND WORKING)

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

* Plain text: Hello world!
* Finding Big Prime numbers
* Calculating N = P * Q = 105061437740618400841751815103485843772900018163207227878597714063924116015569567946519129138173427894806320898500493646912347868102096725176100448023741769777882646066357540662156140732678563360778999550201179680099035509145303572645350787641604231605334738143959611745162153224524225089447791769349840341611
Generating key pair...

* Public key is:  65537
* Private key is: 16598350952383583659848609084661983710340827136760114705509875816986897435421323931798206556550462676393863719472574442225467290634742351533841097987973542760618925920396000166372201814567779108753258829070069521088628498119210909499687585288677905420966448619507997857592257097762834973885352153403674268017
* Decrypted Hello world!

### -----------------------------------

## Miller-Rabin

* A probabilistic algorithm to determine if n is prime or not.
* For any prime p, a^p-1 = 1 mod p for 1 < a < p. This is Fermat's
Little Theorem.
* Input n and k are provided where n is greater than 2 and k is number of rounds of testing which determines the accuracy of test, more the rounds, more the accuracy
* Output: "composite" if n is composite, otherwise "probably prime"
