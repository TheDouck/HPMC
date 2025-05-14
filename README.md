# **HPMC**

>[!IMPORTANT]
>HPMC is currently a prototype to show the ability for Hexium to download files from the internet.

---
Welcome to **HPMC (Hexium Package Managing Client)**, a package manager that is designed to work with the Hexium Operating System.

The package manager is written in Rust and designed to download and install packages from different sources. It uses a package name system that looks like the **emerge** package manager.

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
#### Download of packages
```
hpmcinstall <catogory>/<package>
```
When inside **HPMC**
```
install
```
When inside the **install** function
```
<owner>/<repo>
```
---
