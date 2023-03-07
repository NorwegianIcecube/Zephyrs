// ¤ state -===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-
let keypair_user = null;
let keypair_program = solanaWeb3.Keypair.generate();
import { endpoint } from "./config.js";
let connection = null;
// About Connection():
//  if you want to use solana-test-validator, in my case, I use endpoint = http://localhost:8899 in config.json
//  if you want to use solana dev net, use solanaWeb3.clusterApiUrl("devnet"), or https://api.devnet.solana.com?

// ¤ functions -===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-
function changeTabTo(tabId) {
    // todo: use querySelector instead
    var tabs = document.getElementById("tabs").children;
    var tab_content = document.getElementById("tab-content").children;

    var i;
    for (i = 0; i < tab_content.length; i++) {

        var tab = tabs[i];
        var content = tab_content[i];

        if (tab.id == tabId) {
            tab.className = "tab-open";
            content.style.display = "block";
        } else {
            tab.className = "tab-closed";
            content.style.display = "none"
        }
    }
}

// ¤ run -===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-===-
var tabs = document.querySelectorAll("#tabs>button");
tabs[0].onclick = function () { changeTabTo(1) };
tabs[1].onclick = function () { changeTabTo(2) };
tabs[2].onclick = function () { changeTabTo(3) };

var input = document.querySelector("#secret-key");
input.placeholder = "[1, 2, 3, ...]";
input.addEventListener("keypress", function (event) {
    if (event.key == "Enter") {
        try {
            connection = new solanaWeb3.Connection(endpoint);
        } catch (e) {
            console.error("couldn't find network at endpoint: ", endpoint, ", see error: ", e);
            return;
        } try {
            keypair_user = solanaWeb3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(input.value)));
        } catch (e) {
            console.warn("invalid secret key, see error: ", e);
            input.placeholder = "invalid key, try again";
            input.value = "";
            return;
        }

        connection.getBalance(keypair_user.publicKey).then(balance => {
            document.querySelector("h1").innerHTML = "welcome!";
            document.querySelector("#balance").innerHTML = "balance: " + balance + " SOL";
            document.querySelector("#not-signed-in").style.display = "none";
            document.querySelector("#signed-in").style.display = "block";
        }).catch(err => {
            console.error("couldn't connect to blockchain, see error: ", err);
        });
    }
});
