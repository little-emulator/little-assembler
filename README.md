<div align="center">

# Little Assembler

[![Rust](https://img.shields.io/badge/Rust-f74c00?logo=rust)](https://www.rust-lang.org)
[![Brain made](https://img.shields.io/badge/Brainmade-grey?logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgd2lkdGg9IjY2LjUzNzY5NyIKICAgaGVpZ2h0PSI3OC43ODIxNTgiCiAgIHZpZXdCb3g9IjAgMCA2Ni41Mzc2OTcgNzguNzgyMTU4IgogICBmaWxsPSJub25lIgogICB2ZXJzaW9uPSIxLjEiCiAgIGlkPSJzdmcxNSIKICAgc29kaXBvZGk6ZG9jbmFtZT0id2hpdGUtbG9nby1oZWFkLnN2ZyIKICAgaW5rc2NhcGU6dmVyc2lvbj0iMS4zLjIgKDA5MWUyMGVmMGYsIDIwMjMtMTEtMjUsIGN1c3RvbSkiCiAgIHhtbG5zOmlua3NjYXBlPSJodHRwOi8vd3d3Lmlua3NjYXBlLm9yZy9uYW1lc3BhY2VzL2lua3NjYXBlIgogICB4bWxuczpzb2RpcG9kaT0iaHR0cDovL3NvZGlwb2RpLnNvdXJjZWZvcmdlLm5ldC9EVEQvc29kaXBvZGktMC5kdGQiCiAgIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICAgeG1sbnM6c3ZnPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPHNvZGlwb2RpOm5hbWVkdmlldwogICAgIGlkPSJuYW1lZHZpZXcxNSIKICAgICBwYWdlY29sb3I9IiNmZmZmZmYiCiAgICAgYm9yZGVyY29sb3I9IiMwMDAwMDAiCiAgICAgYm9yZGVyb3BhY2l0eT0iMC4yNSIKICAgICBpbmtzY2FwZTpzaG93cGFnZXNoYWRvdz0iMiIKICAgICBpbmtzY2FwZTpwYWdlb3BhY2l0eT0iMC4wIgogICAgIGlua3NjYXBlOnBhZ2VjaGVja2VyYm9hcmQ9IjAiCiAgICAgaW5rc2NhcGU6ZGVza2NvbG9yPSIjZDFkMWQxIgogICAgIHNob3dncmlkPSJmYWxzZSIKICAgICBpbmtzY2FwZTp6b29tPSIyLjYyNzU2NTQiCiAgICAgaW5rc2NhcGU6Y3g9IjE2LjE3NDY2OSIKICAgICBpbmtzY2FwZTpjeT0iODUuMjUwMDE4IgogICAgIGlua3NjYXBlOndpbmRvdy13aWR0aD0iMTUzNiIKICAgICBpbmtzY2FwZTp3aW5kb3ctaGVpZ2h0PSI3OTIiCiAgICAgaW5rc2NhcGU6d2luZG93LXg9IjAiCiAgICAgaW5rc2NhcGU6d2luZG93LXk9IjAiCiAgICAgaW5rc2NhcGU6d2luZG93LW1heGltaXplZD0iMSIKICAgICBpbmtzY2FwZTpjdXJyZW50LWxheWVyPSJnMTUiIC8+CiAgPGcKICAgICBjbGlwLXBhdGg9InVybCgjY2xpcDBfNF8yMykiCiAgICAgaWQ9ImcxNSIKICAgICBzdHlsZT0iZmlsbDojMDAwMDAwIgogICAgIHRyYW5zZm9ybT0idHJhbnNsYXRlKC0xLjg4MTk3MjZlLTcsLTIuNDQxNDFlLTQpIj4KICAgIDxwYXRoCiAgICAgICBkPSJNIDUyLjYxMiw3OC43ODI0IEggMjMuMzMgYyAtMS40MTM0LDAgLTIuNTU5MSwtMS4xNDU2IC0yLjU1OTEsLTIuNTU4OSB2IC03LjY3NjggaCAtNy45NzQgYyAtMS40MTMzLDAgLTIuNTU5MSwtMS4xNDU3IC0yLjU1OTEsLTIuNTU5IFYgNTUuMzEzNSBMIDEuNDE2NjYsNTAuOTE3MiBDIDAuMDM4MjIwNSw1MC4yMjkxIC0wLjQyNDEwNyw0OC40ODggMC40MzE0MTEsNDcuMjA2OCBMIDEwLjIzNzgsMzIuNDkyOCBWIDI4LjE0MjYgQyAxMC4yNDEyLDEyLjU5OSAyMi44NDMzLDIuNDQxNDFlLTQgMzguMzg3OCwyLjQ0MTQxZS00IDUzLjkzMjIsMi40NDE0MWUtNCA2Ni41MzQ0LDEyLjU5OSA2Ni41Mzc3LDI4LjE0MjYgNjUuOTA2Myw1NS45NjcxIDU1Ljc3NzQsNTEuNjU5NCA1NS4zNTcxLDYyLjE4ODQgbCAtMC4xODYsMTQuMDM1MSBjIC0wLjAwOTQsMC43MDY2IC0wLjI4NjUsMS4zNDY0IC0wLjc0OTYsMS44MDk0IC0wLjQ2MzEsMC40NjMxIC0xLjEwMjgsMC43NDk1IC0xLjgwOTUsMC43NDk1IHogTSAyNS44ODkxLDczLjY2NDUgSCA1MC4wNTI5IEwgNTAuMzM4Niw1OS4xMjMxIEMgNTAuMDc2MSw1Mi40NjcgNjIuMDU0OSw1MC44Nzk1IDYxLjQxOTYsMjguMzg4NSA2MS4wNjA1LDE1LjY3NjMgNTEuMTA1OCw1LjExNzU3IDM4LjM4NzgsNS4xMTc1NyAyNS42Njk4LDUuMTE3NTcgMTUuMzU5LDE1LjQyNTMgMTUuMzU2LDI4LjE0MjYgdiA1LjExNzkgYyAtOGUtNCwwLjUwNTMgLTAuMTUwMywwLjk5OTMgLTAuNDI5OSwxLjQyMDIgbCAtOC42MjkyNiwxMi45NDA2IDcuNjQzOTYsMy44MjA1IGMgMC44Njc3LDAuNDMzNiAxLjQxNTcsMS4zMjA0IDEuNDE1MiwyLjI5MDMgdiA5LjY5NjcgaCA3Ljk3NCBjIDEuNDEzMywwIDIuNTU5MSwxLjE0NTYgMi41NTkxLDIuNTU4OSB6IgogICAgICAgZmlsbD0id2hpdGUiCiAgICAgICBpZD0icGF0aDEiCiAgICAgICBzdHlsZT0iZmlsbDojZmZmZmZmIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Ik0gNDAuMzcyMSw1OC4yMjE5IFYgMzguOTMzNSBjIDAuMTE4MywwIDAuMjM2NiwwLjAxODkgMC4zNTQ5LDAuMDE4OSA5Ljc2ODgsLTAuMDEyNyAxNy4wNDk4LC05LjAxMiAxNS4wMjI0LC0xOC41Njc2IEMgNTUuNTU1LDE5LjQ3MTMgNTQuODQxMywxOC43NTc2IDUzLjkyNzgsMTguNTYzMyA0NS44MjE2LDE2LjgzMjEgMzcuODA2NiwyMS44NTQ2IDM1LjgyOTksMjkuOTA0IDM1LjgwNjQsMjkuODgwNCAzNS43ODczLDI5Ljg1NDMgMzUuNzYzNywyOS44MzA2IDMyLjA5LDI2LjE0MjUgMjYuNzk0OSwyNC41NzE4IDIxLjcwNDIsMjUuNjYwMSBjIC0wLjkxMzYsMC4xOTQzIC0xLjYyNzMsMC45MDggLTEuODIxNiwxLjgyMTUgLTIuMDI3Myw5LjU1NDYgNS4yNTIxLDE4LjU1MzMgMTUuMDIsMTguNTY3NiAwLjIzNjYsMCAwLjQ5MjEsLTAuMDI4NCAwLjczODEsLTAuMDQwMiBWIDU4LjIyMTkgWiBNIDQzLjIxMSwyNi4wNzg4IGMgMi4xNDIyLC0yLjE0NDcgNS4wOTcyLC0zLjI3NDUgOC4xMjM5LC0zLjEwNiAwLjM0OTIsNi4zMzk5IC00Ljg4NzQsMTEuNTc3MyAtMTEuMjI3NywxMS4yMjk1IC0wLjE3NzQsLTMuMDI3NSAwLjk1MjgsLTUuOTg1NSAzLjEwMzgsLTguMTIzNSB6IE0gMjcuNDAzMiwzOC4xOTMxIGMgLTIuMTUzMywtMi4xMzc0IC0zLjI4ODMsLTUuMDk0NCAtMy4xMTgsLTguMTIzNSA2LjM0NCwtMC4zNTc1IDExLjU4NjYsNC44ODU5IDExLjIyNzcsMTEuMjI5NSAtMy4wMjM0LDAuMTY5NiAtNS45NzI4LC0wLjk2MDYgLTguMTA5NywtMy4xMDYgeiIKICAgICAgIGZpbGw9IndoaXRlIgogICAgICAgaWQ9InBhdGgyIgogICAgICAgc3R5bGU9ImZpbGw6I2ZmZmZmZiIgLz4KICA8L2c+CiAgPGRlZnMKICAgICBpZD0iZGVmczE1Ij4KICAgIDxjbGlwUGF0aAogICAgICAgaWQ9ImNsaXAwXzRfMjMiPgogICAgICA8cmVjdAogICAgICAgICB3aWR0aD0iMjU2IgogICAgICAgICBoZWlnaHQ9IjgwIgogICAgICAgICBmaWxsPSIjZmZmZmZmIgogICAgICAgICBpZD0icmVjdDE1IgogICAgICAgICB4PSIwIgogICAgICAgICB5PSIwIiAvPgogICAgPC9jbGlwUGF0aD4KICA8L2RlZnM+Cjwvc3ZnPgo=)](https://brainmade.org)
[![GNU AGPLv3.0 License](https://img.shields.io/badge/License-GNU%20AGPLv3.0-dark_green?logo=gnu)](https://choosealicense.com/licenses/agpl-3.0)
[![Buymeacoffee](https://img.shields.io/badge/Buymeacoffee-gray?logo=buymeacoffee)](https://buymeacoffee.com/nicolabelluti)
<br>
[![CI Badge](https://git.nicolabelluti.me/little-emulator/little-assembler/actions/workflows/check-format-and-test.yaml/badge.svg)](https://git.nicolabelluti.me/little-emulator/little-assembler/actions/?workflow=check-format-and-test.yaml)
[![GitHub Stars](https://img.shields.io/github/stars/little-emulator)](https://github.com/little-emulator)

</div><br>

> An assembler library for the LC2 ISA 🏗️

## Library Usage

1. Add the library to you project:

   ```shell
   cargo add assemblers --git https://git.nicolabelluti.me/little-emulator/little-assembler.git
   ```

2. Use the assembler in you project:

   ```rust
   use assemblers::{lc2::Lc2AssemblerBuilder, Assembler};
   
   fn main() {
       // Create a new assembler
       let assembler = Lc2AssemblerBuilder::new().build();
   
       // Assemble
       let (binary, symbol_table) = assembler.assemble(r#"
           .orig 0x3000
           
           LEA R0, string
           PUTS
           HALT
   
           string: .stringz "Hello, World!"
           
           .end
       "#).unwrap();
   
       // Print the binary
       for byte in binary {
           print!("{:02x}", byte);
       }
       println!();
   }
   ```

### Builder setters

| Param | Default | Description |
|:-:|:-:|:-|
| `.optional_starting_orig(...)` | `false` | Allow the assembly to start witout a `.orig` directive. The start address will be set at `0` |
| `.multiple_origs(...)` | `false` | Allow the assembly to have more than one `.orig` directive. They must be declared in order |
| `.optional_end(...)` | `false` | Allow the assembly to end without a `.end` directive |
| `.nothing_after_end(...)` | `true` | Return an error if the assembly has some instructions after the `.end` directive |
| `.enable_stringzp(...)` | `false` | Enable the `.STRINGZP` pseudo-operation to create null-terminated packed strings |
| `.prepend_start_address(...)` | `true` | Add the starting address to the start of the binary |
