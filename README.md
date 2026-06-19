## PNG Encoder/Decoder Package with CLI

Built with Rust following these specs: https://jrdngr.github.io/pngme_book/introduction.html

# Installation:
` cargo install --path . `

# Usage:
To encode "This is a secret message!" in dice.png file in data chunk ruSt:

` pngme encode ./dice.png ruSt "This is a secret message!" `

To decode a message in data chunk ruSt:

` pngme decode ./dice.png ruSt `

To remove a message in data chunk ruSt:

` pngme remove ./dice.png ruSt `

To print the bytes in dice.png file:

` pngme print ./dice.png `
