function copyToClipboard() {
    var urlText = document.getElementById("one_time_url");
    var CopyRange = document.createRange();
    CopyRange.selectNode(urlText);
    window.getSelection().addRange(CopyRange);
    document.execCommand("copy");
    window.getSelection().removeRange(CopyRange);
}
