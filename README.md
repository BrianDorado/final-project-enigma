# Enigma Machine Emulator in Rust
## Overview
This project is a Rust-based emulator of the Enigma Machine, a famous cryptographic tool used primarily by Nazi Germany during World War II. The Enigma Machine was a complex device that encrypted messages by substituting letters using a series of rotors and a plugboard. This emulator aims to replicate the functionality of the original Enigma Machine, allowing users to encrypt and decrypt messages using the same mechanisms.

## How the Enigma Machine Works
The Enigma Machine operates using a combination of rotors and a plugboard:

Rotors: These are disk-like components with wiring that scrambles the alphabet. Each rotor has 26 positions, corresponding to the letters of the alphabet. The Enigma Machine typically had three or more rotors. As a key is pressed, the rotors rotate, changing the wiring path and thus the substitution for each letter.

~~Plugboard: Before reaching the rotors, each letter input is first passed through the plugboard, which swaps certain letters. For example, if A and B are connected on the plugboard, all As will become Bs and vice versa.~~(Not yet implemented)

Reflector: At the end of the rotors, the signal is sent through a reflector that sends the signal back through the rotors by a different route, further scrambling the letter.

Output: The letter that emerges from the final rotor is the encrypted version of the input letter.

## Code Implementation:

Rotor Implementation: The largest part of the code, these emulate the behavior of the Enigma rotors, including their ability to rotate and the substitution they perform. There are 5 Rotors and they can be placed in any configuration (they do not have to be placed in order). 

~~Plugboard Implementation: This section simulates the plugboard of the Enigma Machine, allowing specific letter substitutions as per the user's configuration.~~

Reflector Implementation: The reflector's functionality is replicated here, ensuring that the signal is reflected back through the rotors. There are three reflectors but only one can in use at a time. They are independent of the Rotory mechanisms and the plugboard. 

Encryption/Decryption Logic: The entire Process can be read here: https://en.wikipedia.org/wiki/Enigma_machine

User Interface: The part of the code that interacts with the user, allowing them to input the message to be encrypted or decrypted and the configuration of the machine (rotor settings, plugboard connections).

## Usage
To use this emulator:

~~Set Up: Configure the initial settings of the Enigma Machine, such as the starting positions of the rotors and the plugboard connections.~~ (Not implemented yet)

Input Message: Enter the message you wish to encrypt or decrypt.

Process Message: The emulator will process the input message through the configured machine, outputting the encrypted or decrypted text. It currently does not handle Punctuation or Special Characters

# Decryption
1. Initial Setup: To decrypt a message, the recipient's Enigma machine had to be set up with the exact configuration as the sender's machine at the time of encryption. This meant setting the rotors to specific positions, choosing the same rotors, and connecting the same pairs of letters on the plugboard.

2. Typing the Encrypted Message: The operator would then type the encrypted message. As each letter was typed, the rotors moved, changing the electrical pathway.

3. Inverse Process: The beauty of the Enigma was that the encryption and decryption processes were essentially the same. The machine's design meant that if an A was encrypted into a B, typing B would decrypt it back to A, provided the machine was in the same initial state.

4. Rotor Movement: Each keypress caused the rightmost rotor to rotate one position, altering the wiring path for the next letter. This continuous rotation meant that the same plaintext letter would be encrypted differently each time, adding a significant level of complexity.

5. Output: As the encrypted letter was typed, the corresponding decrypted letter would light up on the lampboard.

For example - the current implementation in this code base encrypts the message "Hello" to "BCUTS". A user, ensuring he has the same initial set up as the originator would type in "BCUTS" and the word "Hello" would light up on the lampboard. 

This feature is not yet supported

# TODO:
- Decryption
- Plugboard
- CLI Interface allowing customiszation of rotors, reflector, interactions with the plugboard and initial position for each rotor
- File (.txt) encryption


