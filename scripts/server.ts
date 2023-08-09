import { Application, Router } from "https://deno.land/x/oak/mod.ts";

const router = new Router();
router
  .get("/", (context) => {
    context.response.body = { "unifiedpush": { "version": 1 } };
  });

const app = new Application();
app.use(async (ctx, next) => {
  await next();
  const rt = ctx.response.headers.get("X-Response-Time");
  console.debug(`${ctx.request.method} ${ctx.request.url} - ${rt}`);
});
app.use(async (ctx, next) => {
  const start = Date.now();
  await next();
  const ms = Date.now() - start;
  ctx.response.headers.set("X-Response-Time", `${ms}ms`);
});
app.use(router.routes());
app.use(router.allowedMethods());


console.info("Listening on 8000 ...");
await app.listen({ port: 8000 });
