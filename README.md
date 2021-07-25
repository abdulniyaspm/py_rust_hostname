# py_rust_hostname

A simple python extension written in Rust to get the name of the localhost.

## Usage example

* Clone the project
* Build the project using ```cargo build --release```
* Navigate to the release folder(```target/release```)
* On MacOS rename ```libpy_rust_hostname.dylib``` to ```py_rust_hostname.so```. On Linux rename ```libpy_rust_hostname.so``` to ```py_rust_hostname.so```(Windows is not supported as this extension use ```libc``` under the hood)
* Now you are ready to import the module. Let's test it!. 

Open your python interpretter

```python
>>> import py_rust_hostname
>>> py_rust_hostname.get_hostname.__doc__
'Return the name of the localhost'
>>> py_rust_hostname.get_hostname()
'<Your Hostname will be displayed here>'
>>> dir(py_rust_hostname)
['__all__', '__doc__', '__file__', '__loader__', '__name__', '__package__', '__spec__', 'get_hostname']
```

## Meta

Abdul Niyas P M – [@AbdulNiyas19](https://twitter.com/AbdulNiyas19) – abdulniyaspm@gmail.com

[Github Profile](https://github.com/abdulniyaspm)