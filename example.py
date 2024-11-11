import fairy
import asyncio
from urllib.parse import urlparse
import urllib.request


class Client(fairy.HttpClient):
  async def send(self, req):

    url  = urlparse(req.uri);
    # Fairy will redirect any relative urls to the internal scheme
    if url.scheme == 'internal':
      return fairy.Response(status = 200, body = bytes('{"hello": "python world"}', 'utf8'))
  
    ret = urllib.request.urlopen(req.uri).read();
    return fairy.Response(status = 200, body = ret)
    



async def main():
  # fairy.uniffi_set_event_loop(asyncio.get_running_loop())

  # Create renderpool. This should only be done one time for every application
  app = await fairy.create(Client(), "./example/config.json")

  # Get a renderer
  # If the frontend app has multiple entrypoints you have to
  # specify which one to use. (eg main, admin).
  # If ther's only one unamed entrypoint, you'll use None
  renderer = app.renderer(None)

  # Send a request to the renderer  
  ret = await renderer.render(fairy.Request(uri = "http://localhost:3000/subpage", body = bytes()))

  print("Assets")
  for asset in ret.assets:
    print(f"  {asset.kind} at {asset.file}")

  print("\nExtra head tags")
  for head in ret.head:
    print(f" {head}")

  print("\nContent")
  print(ret.content)
  

if __name__ == "__main__":
  import time
  s = time.perf_counter()
  try:
    asyncio.run(main())
  except Exception as e:
    print(e)
    print("Failed")
  elapsed = time.perf_counter() - s
  print(f"{__file__} executed in {elapsed:0.2f} seconds.")
