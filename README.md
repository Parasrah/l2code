# Setup

## Windows

1) Install [vscode](https://code.visualstudio.com/)
  * will work for most programming languages
  * will work on less powerful hardware
  * has extensions to create sharing sessions
2) Install [vscode live share pack](https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare-pack)

> At this point you should be able to follow along the [Lessons](#lessons) section with your host

3) Install [rustup](https://rustup.rs/)
4) Install the [rls vscode extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
5) Install [git](https://git-scm.com/)
6) Open a command prompt (search for `cmd`)
7) Clone this repository onto your machine with the command `git clone https://github.com/Parasrah/l2code.git`
  * Ask your host for directions on how to clone the repository to a specific location on your computer
8) In vscode, press Ctrl+Shift+P, write `file: open folder`, click enter and open the new `l2code` directory you just cloned

## Mac & Linux

> This is not yet implemented and may change!!

1) Open a terminal
2) Follow the [quick start](https://nixos.org/nix/manual/#chap-quick-start) section to get the Nix package manager installed
3) If you don't have `git` installed, install it with the command `nix-env -i git`
4) Clone this repository onto your machine with the command `git clone https://github.com/Parasrah/l2code.git`
  * this will create a new directory in your current directory called `l2code`
  * if you don't know what `cd` does, ask your host to explain it or read [this](http://linuxcommand.org/lc3_lts0020.php) (applies to macs as well)
5) Move into the new `l2code` directory w/ `cd l2code`
6) Run the command `nix-shell`
  * this will download the dependencies necessary to work on the exercises
  * every time you start working on the project, you will have to run this command in the `l2code` directory first
7) Run the command `code`
  * this should open an instance of Visual Studio Code

# Lessons

Follow along with your host through the `main.rs` file

# Exercises



# Navigating to issues

![goto issues](/images/issues.png)

# Why did a PR fail?

![goto pr](/images/pr.png)

![goto failures](/images/in_pr.png)

![failures](/images/failures.png)
