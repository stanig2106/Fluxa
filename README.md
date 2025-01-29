# Fluxa

### Homemade web browser

The goal of this project is to create a web browser from scratch,
using rust, without external librairies.

## Architecture

Flux-core: common code for the whole project.
Fluxa-network: 

```text
+~~~~~~~~~~~~~~~~~+
| flux-core       |
| (common code)   |
+~~~~~~~~~~~~~~~~~+
       ||
       vv
+--------------------+    +-------------------+    
| fluxa-network      |--->| fluxa-parser      |
| (HTTP, HTTPS, etc.)|    | (HTML, CSS, …)    |
+--------------------+    +---------+---------+
  ^                            /    |
  |       +-------------------+     |
  |       |                         |
  |       v                         v
  |    fluxa-dom   <---------->  fluxa-js <----------+
  |       |                        |                 |
  |       |                        |                 |
  |       +---------+--------------+                 |
  |                 |                                |
  |                 v                                |
  |     fluxa-renderer (layout, styles, etc.)        |
  |                 |                                |
  |                 v                                |
  +---  fluxa-ui (windowing, event loop, etc.)  -----+
```
