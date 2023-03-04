import { createProxyMiddleware } from 'http-proxy-middleware'

const proxyMiddleware = createProxyMiddleware((path, req) => {
  return Boolean(path.match(/^\/oauth2\/authorize\/?$/)) && req.method === 'POST'
}, {
  target: 'http://127.0.0.1:8000/',
  changeOrigin: true,
})

export default defineEventHandler(async (event) => {
  await new Promise((resolve, reject) => {
    // @ts-expect-error: Request extends IncomingMessage
    proxyMiddleware(event.node.req, event.node.res, (err) => {
      if (err) reject(err)
      else resolve(true)
    })
  })
})