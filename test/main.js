const { Application } = require("../index");
const app = new Application({
  path: "./test/index.html",
  width: 700,
  height: 700,
  title: "Wishin is here!"
});

app.run();
