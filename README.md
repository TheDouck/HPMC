# **HPMC**

>[!IMPORTANT]
>HPMC is currently a prototype to show the ability for Hexium to download files from the internet.

>[!WARNING]
>**HPMC DOES NOT WORK.**
>
>The current implementation does **not** properly select mirrors by priority, does **not** parse direct URLs from the mirrorlist, and may not handle downloads correctly. Many features are incomplete or missing.  
>
>Use this code for demonstration or development purposes only.

---
Welcome to **HPMC (Hexium Package Managing Client)**, a package manager that is designed to work with the Hexium Operating System.

The package manager is written in Rust and designed to download and install packages from different sources.

>[!IMPORTANT] 
>We currently don't have a way to download and install non pre-compiled software.
---
## :hammer: Builiding
You can build the package manager using the `cargo` tool.

```
cargo build
```
---
## :rocket: Usage
```
HPMC
```
### When inside **HPMC**
#### Download of packages
```
install <package>
```
#### Exiting
```
exit / quit
```
#### Clear screen
```
clear
```
#### Help
```
help
```
#### About
```
about
```
