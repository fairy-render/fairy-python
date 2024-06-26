import fairy
import asyncio


class Client(fairy.HttpClient):
  def send(self, req):
    return fairy.Response(status = 200, body = bytes('{"hello": "world"}', 'utf8'))


def main():

  app = fairy.create(Client(), "../fairy-render/solid-config.json")

  renderer = app.renderer(None)

  ret = renderer.render(fairy.Request(uri = "/", body = bytes()))


  #print(ret)
  

if __name__ == "__main__":
  import time
  s = time.perf_counter()
  try:
    main()
  except:
    print("Failed")
  elapsed = time.perf_counter() - s
  print(f"{__file__} executed in {elapsed:0.2f} seconds.")