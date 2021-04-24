import yosay from 'yosay';

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  try {
    const module = await import(__dirname + "/../pkg");
    const output = module.parse(request)

    let res = new Response(output, { status: 200 })
    res.headers.set('Content-type', 'text/html')
    return res
  } catch (e) {
    console.error(e);
    return new Response(yosay("error"), { status: 500 })
  }
}