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