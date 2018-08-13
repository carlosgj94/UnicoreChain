<img src="https://github.com/carlosgj94/UnicoreChain/blob/master/logo.png" align="left" height="300" width="300">

 # UnicoreChain
 Basic Blockchain to rocket your project
 
Minimum viable blockchain to start your very own. The objective is to keep it as simple as possible so anyone can focus in a certain part of the developement without having to worry about the parts that are not that exciting for that person.
Because of that reason, each part of the project is divided in small sections for easy upgradeability.

## Running
Since we are using Rust language, running the project is pretty straightforward. Simply run:```$ cargo run``` in the project root. That's it.
## Parts
The project consists in very different modules that get tied to work together as a blockchain. This is done in this way so anyone can help in one part without messing other person work providing future upgradeability in an easy way.

The main parts are:
- Chain: It has the data structures and functionality to store the transactions and the blocks tying all of them together.
- CLI: It gets the information of the terminal to communicate with the module needed.
- DB: It stores the data. This part will save the blockchain in local and will provide the funcitons to access the data.
- Keys: The keys are the addresses of each person/account. This module mission is to create and provide the minimum funcitonality of these keys to work.
- Miner: The miner will handle the mission of forming the transactions by solving a puzzle. In our case, in this first version the chosen consensus algorithm is Proof Of Work, as bitcoin does.
- P2P: It has the server and functionality to connect with other nodes and spread information.
- Primitives: Basic data structures such as Hash data.
- Sync: Handles the synchronization between nodes.

## Helping
We need help to make our 🦄 alive. 

If you don't help us, it is considered assassination of an endangered animal. So help us. Please.
We want ANY kind of developer to help us so we are prividing an easy way for them to start helping us.
1. You can join our development chats. Right now, since it is still a VERY VERY early days project we haven't opened one yet, but you can use the Core Dumped general chat (spanish) --> https://t.me/CoreDumpedUPM
2. Execute the code. Since we use Rust it is pretty easy! Read the Running part in this readme to get the details.
3. Checking out the current bugs tagged as easy for noobs --> https://github.com/carlosgj94/UnicoreChain/issues?q=is%3Aopen+is%3Aissue+label%3AEasy
4. At the beginning, even the most easy issues must be hard for you. If that is your case, come to the chat and ask us personally for help, or write in the chat of the issue here in GitHub, we will reply with anything you need there.
5. Read the book Mastering Bitcoin of Andreas Antonopoulos. He is a genious. This code is very inspired by his book, so reading it you will feel closer to the code. It is also important the Bitcoin Whitepaper by Satoshi Nakamoto.
6. There are a few Rust blockchain implementations that deserve to take a look to get inspired. The first one is Parity's implementation: https://github.com/paritytech/parity-bitcoin. The second one is Grin, that has way more complicated stuff, but might help you for more technical implementations you would like to program or take a look: https://github.com/mimblewimble/grin.
 Alternatively to solving a bug, you can "just" start adding more tests to the current implementation. Tests are VERY important and fun, and they also provide a strong view of how the code actually works, so I really recommend you to try to write wider test to the ones we have in one of the modules. Can you try to get to a 100% code coverage for the 🦄?
