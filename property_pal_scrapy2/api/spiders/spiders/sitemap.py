from scrapy.spiders import XMLFeedSpider
from scrapy.selector import Selector
from scrapy import Request
import os
import sys
import django
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(project_root)
os.environ['DJANGO_SETTINGS_MODULE'] = 'api.settings'
django.setup()
from backend.models import Property


class SitemapSpider(XMLFeedSpider):
    name = "sitemap"
    start_urls = [
            "https://www.propertypal.com/sitemaps/town/property-for-sale/index",
            "https://www.propertypal.com/sitemaps/town/property-to-rent/index"
        ]
    iterator = "iternodes"  # you can change this; see the docs
    itertag = "sitemap"  # change it accordingly
    urls = []

    def parse_node(self, response, selector: Selector):
        yield from self.parse_propertypal(response, selector)
            
    def parse_propertypal(self, response, selector: Selector):
        item = {}
        namespace = {'ns': 'http://www.sitemaps.org/schemas/sitemap/0.9'}
        item["url"] = selector.xpath("ns:loc/text()", namespaces=namespace).get()
        yield Request(
            item["url"],
            callback=self.parse_propertypal_layer2,
            meta={"ns": namespace}
        )   
    
    def parse_propertypal_layer2(self, response):
        selector = Selector(response)
        namespace = response.meta["ns"]
        item = {}
        item["url"] = selector.xpath(".//ns:loc/text()", namespaces=namespace).getall()
        for idx, thing in enumerate(item["url"]):
            item["url"][idx] = (thing, f"{thing.split('/')[-1]}")
        self.urls.extend(item["url"])
        
    def closed(self, reason):
        self.close_spider(reason)
    
    def close_spider(self, reason):
        Property.objects.bulk_create(
            [Property(
                property_id = item[-1],
                url = item[0]
            ) for item in self.urls],
            ignore_conflicts=True
        )
