const rust = import('./pkg')

rust
  .then(m => m.say_hello())
  .catch(console.error)