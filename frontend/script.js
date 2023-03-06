import * as solanaWeb3 from "https://unpkg.com/@solana/web3.js@latest/lib/index.iife.js";
var keypair = null;
function changeTabTo(tabId) {

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


/// check for public key given private key
function trySignIn(private_key) {

    console.warn("this function is not working yet, try setting parametre as \"secret key\"");
    try {
        keypair = solanaWeb3.Keypair.fromSecretKey(private_key);
    } catch (e) {
        console.error(e);
    }
    if (private_key == "secret key") {
        return true;
    } else { return false; }
}

var input = document.querySelector("#secret-key");;
input.addEventListener("keypress", function (event) {
    if (event.key == "Enter") {
        var status = trySignIn(input.value);
        if (status == true) {
            document.querySelector("#not-signed-in").style.display = "none";
            document.querySelector("#signed-in").style.display = "block";
        } else {
            input.placeholder = "invalid key, try again";
            input.value = "";
        }
        ;
    }
});

var tabs = document.querySelectorAll("#tabs>button");
tabs[0].onclick = function () { changeTabTo(1) };
tabs[1].onclick = function () { changeTabTo(2) };
tabs[2].onclick = function () { changeTabTo(3) };