## requirements
To test this you need:
- solana-test-validator
- a solana account
- your secret key
    - run `solana get config`
    - it will show you the location of `id.json`
    - grab the *u8 array* secret key, you will need it in the web page
- make sure `setup.js` has the same endpoint as solana-test-validator uses
    - I'm not sure where to find this
- a browser

Open `index.html` in a browser.