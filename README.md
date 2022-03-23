# Luhn (Mod 10) Algorithm in Rust

> This repository is purely for reference and is illustrative in it is purpose. This is one of the many ways of implementing this algorithm. 


This project illustrates an implementation of the [Luhn Algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) in Rust. The Luhn Algorithm is the 
standard method of validating a credit card number prior to authorization. It is also an interesting algorithm as it teaches
a lot of the basics and nuances of a language through implementation. Still learning Rust at a moderate pace, I will be updating this repo as I come across
implementation details that may benefit readability and execution.


## Prerequisites

Before you continue, ensure you have met the following requirements:

* **rustup** must be installed

## Testing

* Issue from the command line **cargo test**


## Notes

* This repository is heavily commented to provide context as to what and why, if in VS Code feel free to collapse all comments if they are obtrusive
    * On Mac -> Press <kbd>&#8984;</kbd> + <kbd>K</kbd> then <kbd>&#8984;</kbd> + <kbd>/</kbd> 
    * On Windows & Linux -> Press <kbd>Ctrl</kbd> + <kbd>K</kbd> then <kbd>Ctrl</kbd> + <kbd>/</kbd> 
* The checkdigits validated are from various sites across the web, there will be more added and augmentation of the algorithm when edge cases are found.