import django, os, sys, json
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(project_root)

os.environ['DJANGO_SETTINGS_MODULE'] = 'api.settings'
django.setup()

from backend.models import * 
from scrapy.crawler import CrawlerProcess
from spiders.spiders.sitemap import SitemapSpider
from spiders.spiders.property_pal import PropertyPalSpider
from django.db.models import Q
import pandas as pd

def crawl_sequentially(process, crawlers):
    if crawlers[0] == SitemapSpider:
        deferred = process.crawl(crawlers[0])
    
    else:
        process.settings["ITEM_PIPELINES"] = {
            "spiders.pipelines.SpidersPipeline": 100,
        }
        filter = Q(published=True)
        properties = list(Property.objects.filter(filter).values_list("property_id", "url"))
        properties = sorted(properties, key=lambda x: x[0])
        mapper = {}
        urls = []
        
        for property in properties:
            mapper[property[1]] = {
                "property_id": property[0],
                "scraped_before": False,
            }
            urls.append(property[1])
        num_urls = len(urls)
        deferred = process.crawl(crawlers[0], mapper=mapper, start_urls=urls, num_urls=num_urls)

    print(crawlers)

    if len(crawlers) > 1:
        deferred.addCallback(lambda _: crawl_sequentially(process, crawlers[1:]))

if __name__ == "__main__":
    crawlers = [SitemapSpider, PropertyPalSpider]

    process = CrawlerProcess(settings={
        "LOG_LEVEL":"INFO",
        "HTTPCACHE_ENABLED":False,
        "HTTPERROR_ALLOWED_CODES": [410,404],
        "CONCURRENT_REQUESTS" : 16,
    })
    
    crawl_sequentially(process=process, crawlers=crawlers)
    process.start()

    data = list(Property.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property = pd.DataFrame.from_dict(test, orient="index")
    property.to_csv("./property.csv")

    data = list(PropertyData.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./property_data.csv")

    data = list(EstateAgent.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./estate_agent.csv")

    data = list(Accreditation.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./accreditation.csv")

    data = list(AgentAccreditation.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./agent_accreditation.csv")

    data = list(ListingHistory.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./listing_history.csv")

    data = list(KeyFeature.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./key_features.csv")

    data = list(Views.objects.all().values())
    test = {}
    for idx, item in enumerate(data):
        test[idx] = item
    property_data = pd.DataFrame.from_dict(test, orient="index")
    property_data.to_csv("./views.csv")