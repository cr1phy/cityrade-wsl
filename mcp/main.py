from fastmcp import fastmcp

mcp = FastMCP("CityradeMCP")


@mcp.tool
async def hello() -> str:
    return "АЛО!!"
