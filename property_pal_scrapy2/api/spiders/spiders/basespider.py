import scrapy
from abc import ABC, abstractmethod
import os
import sys
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(project_root)
from spiders.pipelines import SpidersPipeline
import time
from scrapy.exceptions import CloseSpider

class BasespiderSpider(ABC, scrapy.Spider):
    name = "basespider"
    allowed_domains = ["fake.com"]
    start_urls = ["https://fake.com"]

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.start_urls = kwargs.get("start_urls", set())
        self.mapper = kwargs.get("mapper", {})
        self.num_urls = kwargs.get("num_urls", 0)
        self.data = []
        self.pipeline = SpidersPipeline()
        self.count = 0
        self.start_time = time.time()
    
    def start_requests(self):
        for url in self.start_urls:
            yield scrapy.Request(url, callback=self.parse, meta = {"db_data":self.mapper[url]})
    
    def check_time_limit(self):
        if (time.time() - self.start_time)/3600 > 20:
            self.close_spider("Time limit exceeded")
    
    @abstractmethod
    def parse(self, response):
        pass
    
    def closed(self, reason):
        self.close_spider(reason)
    
    def close_spider(self, reason):
        self.pipeline.process_item(self.data, self)
        if (time.time() - self.start_time)/60 > 0.25:
            raise CloseSpider(f"It's fucked mate: {reason}")
        
