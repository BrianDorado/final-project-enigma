# Enigma Machine Emulator in Rust
## Overview
This project is a Rust-based emulator of the Enigma Machine, a famous cryptographic tool used primarily by Nazi Germany during World War II. The Enigma Machine was a complex device that encrypted messages by substituting letters using a series of rotors and a plugboard. This emulator aims to replicate the functionality of the original Enigma Machine, allowing users to encrypt and decrypt messages using the same mechanisms.

# How the Enigma Machine Works
The Enigma Machine operates using a combination of rotors and a plugboard:

Rotors: These are disk-like components with wiring that scrambles the alphabet. Each rotor has 26 positions, corresponding to the letters of the alphabet. The Enigma Machine typically had three or more rotors. As a key is pressed, the rotors rotate, changing the wiring path and thus the substitution for each letter.

Plugboard: Before reaching the rotors, each letter input is first passed through the plugboard, which swaps certain letters. For example, if A and B are connected on the plugboard, all As will become Bs and vice versa.(Not yet implemented)

Reflector: At the end of the rotors, the signal is sent through a reflector that sends the signal back through the rotors by a different route, further scrambling the letter.

Output: The letter that emerges from the final rotor is the encrypted version of the input letter.

## Code Implementation:

Rotor Implementation: The largest part of the code, these emulate the behavior of the Enigma rotors, including their ability to rotate and the substitution they perform. There are 5 Rotors and they can be placed in any configuration (they do not have to be placed in order). 

~~Plugboard Implementation: This section simulates the plugboard of the Enigma Machine, allowing specific letter substitutions as per the user's configuration.~~

Reflector Implementation: The reflector's functionality is replicated here, ensuring that the signal is reflected back through the rotors.There are three reflectors but only one can in use at a time. They are independent of the Rotory mechanisms and the plugboard. 

Encryption/Decryption Logic: The entire Process can be read here: https://en.wikipedia.org/wiki/Enigma_machine

User Interface: The part of the code that interacts with the user, allowing them to input the message to be encrypted or decrypted and the configuration of the machine (rotor settings, plugboard connections).

## Usage
To use this emulator:

~~Set Up: Configure the initial settings of the Enigma Machine, such as the starting positions of the rotors and the plugboard connections.~~ (Not implemented yet)

Input Message: Enter the message you wish to encrypt or decrypt.

Process Message: The emulator will process the input message through the configured machine, outputting the encrypted or decrypted text. It currently does not handle Punctuation or Special Characters
