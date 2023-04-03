# **bible.rs**
This is a simple command line tool that accesses [bible-api.com](https://bible-api.com/) to print Bible verses in the terminal

## **Installation**
```
brew tap wzid/tap/bible
brew install bible
```

## **Use**
```
bible Romans 3:23
```
This should output:
```
Romans 3:23
(23) for all have sinned, and fall short of the glory of God;
```
You can also get verse ranges like so:
```
bible Romans 3:23-27
```
**Output:**
```
Romans 3:23-31
(23) for all have sinned, and fall short of the glory of God;
(24) being justified freely by his grace through the redemption that is in Christ Jesus;
(25) whom God sent to be an atoning sacrifice, through faith in his blood, for a demonstration of his righteousness through the passing over of prior sins, in God’s forbearance;
(26) to demonstrate his righteousness at this present time; that he might himself be just, and the justifier of him who has faith in Jesus.
(27) Where then is the boasting? It is excluded. By what kind of law? Of works? No, but by a law of faith.
```


## **TODO**
- Add [`clap_complete`](https://docs.rs/clap_complete/latest/clap_complete/) crate for auto-completion in the terminal
- Switch between translations
