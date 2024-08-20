const folders = document.getElementsByClassName("folder-crochet");
for (let f of folders) {
  const title = f.children[0];
  const itemList = f.children[1];
  title.addEventListener("click", () => {
    const isHidden = itemList.getAttribute("hidden");
    if (isHidden) {
      itemList.removeAttribute("hidden");

    } else {
      itemList.setAttribute("hidden", true);
    }
  })
}