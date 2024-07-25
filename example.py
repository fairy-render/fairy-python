import fairy
import asyncio
from urllib.parse import urlparse
import urllib.request


class Client(fairy.HttpClient):
  async def send(self, req):
    url  = urlparse(req.uri);

    if url.scheme == 'internal':
      return fairy.Response(status = 200, body = bytes('{"hello": "python world"}', 'utf8'))
    
    ret = urllib.request.urlopen(req.uri).read();

    return fairy.Response(status = 200, body = ret)
    



async def main():

  app = await fairy.create(Client(), "../fairy-render/solid-config.json")

  renderer = app.renderer(None)

  ret = await renderer.render(fairy.Request(uri = "/", body = bytes()))


  print(ret)
  

if __name__ == "__main__":
  import time
  s = time.perf_counter()
  try:
    asyncio.run(main())
  except:
    print("Failed")
  elapsed = time.perf_counter() - s
  print(f"{__file__} executed in {elapsed:0.2f} seconds.")