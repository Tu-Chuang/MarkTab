import asyncio
import aiohttp
import time
from datetime import datetime

async def login(session, base_url):
    async with session.post(f"{base_url}/auth/login", json={
        "email": "test@example.com",
        "password": "password123"
    }) as resp:
        return await resp.json()

async def get_profile(session, base_url, token):
    async with session.get(
        f"{base_url}/user/profile",
        headers={"Authorization": f"Bearer {token}"}
    ) as resp:
        return await resp.json()

async def run_test(base_url, num_requests):
    async with aiohttp.ClientSession() as session:
        # 先登录获取token
        login_resp = await login(session, base_url)
        token = login_resp["data"]["access_token"]
        
        start_time = time.time()
        tasks = []
        
        # 创建并发请求
        for _ in range(num_requests):
            tasks.append(get_profile(session, base_url, token))
        
        # 等待所有请求完成
        results = await asyncio.gather(*tasks)
        end_time = time.time()
        
        # 计算统计信息
        duration = end_time - start_time
        success_count = sum(1 for r in results if r["code"] == 1)
        rps = num_requests / duration
        
        print(f"Load Test Results ({datetime.now()}):")
        print(f"Total Requests: {num_requests}")
        print(f"Duration: {duration:.2f} seconds")
        print(f"Successful Requests: {success_count}")
        print(f"Failed Requests: {num_requests - success_count}")
        print(f"Requests per second: {rps:.2f}")

if __name__ == "__main__":
    BASE_URL = "http://localhost:8080"
    NUM_REQUESTS = 1000
    
    asyncio.run(run_test(BASE_URL, NUM_REQUESTS)) 