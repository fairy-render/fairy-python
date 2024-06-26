from typing import Optional, Protocol, Dict, Enum, List;

async def create(client: HttpClient, path: str) -> Fairy: ...

class Fairy:
  def renderer(self, entry: Optional[str]) -> Renderer: ...

class Method(Enum):
  Get = 'Get'
  Post = 'Post'
  Put = 'Put'
  Delete = "Delete"

class Renderer:
  async def render(self, req: Request) -> RenderResult: ...

class Request:
  uri: str
  method: str
  body: bytes
  headers: Dict[str, str]

class AssetKind(Enum):
  Script = "Script"
  Styling = "Styling"
  Unknown = "Unknown"

class Asset:
  kind: AssetKind
  path: str

class RenderResult:
  content: str
  assets: List[Asset]
  head: List[str]

class Response:
  status: int
  body: bytes

class HttpClient(Protocol):
  def send(self) -> Response:
    pass