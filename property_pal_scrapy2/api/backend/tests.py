from django.test import TestCase
import os, sys, django, json
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(project_root)
from backend.models import *
# Create your tests here.

class Test(TestCase):
    def setUp(self) -> None:
        file_path = "./data"
        self.test_data = []
        for item in os.listdir(file_path):
            with open(f"{file_path}/{item}", "r") as f:
                data = json.load(f)
                self.test_data.append(data)

    # def test_big(self):
    #     agents = []
    #     accreditations = []
    #     agentaccreditations = []
    #     one_to_ones = []
    #     listing_histories = []
    #     key_features = []
    #     views = []

    #     properties = []
    #     for item in self.test_data:
    #         if not item["props"]["pageProps"]["published"]:
    #             continue

    #         property_section: dict = item["props"]["pageProps"].get("property", {}) or {}
    #         agent_section: dict = property_section.get("account", {}) or {}
    #         image_section: list = property_section.get("images", []) or []
    #         video_section: list = property_section.get("videos", []) or []
    #         listing_history_section: list = property_section.get("history") or []
    #         key_feature_section: list = property_section.get("keyInfo", []) or []
    #         view_section: dict = property_section.get("stats", {}) or {}
    #         view_section: list = view_section.get("history", []) or []
            
    #         style_section = property_section.get("style", {}) or {}
    #         property_id = property_section.get("pathId")
    #         properties.append(property_id)

    #         if agent_section.get("accountNumber"):
    #             agent = {
    #                 "account_number" : agent_section.get("accountNumber"),
    #                 "agent_name" : agent_section.get("organisation", None),
    #                 "branch_name" : agent_section.get("organisationBranch", None),
    #                 "display_address" : agent_section.get("displayAddress", None),
    #                 "agent_url" : agent_section.get("websiteUrl", None),
    #                 "private_developer" : agent_section.get("privateDeveloper", None),
    #                 "developer_primary_contact" : agent_section.get("developerPrimaryContact", None),
    #                 "developer" : agent_section.get("developer", None),
    #                 "agent_tier" : agent_section.get("tier", None),
    #                 "agent_enhanced_branding" : agent_section.get("enhancedBranding", None),
    #             }
    #             agents.append(agent)

    #         if agent_section.get("accreditations", []):
    #             for element in agent_section["accreditations"]:
    #                 accreditation = {
    #                     "accreditation_id" : element["id"],
    #                     "label" : element["label"],
    #                     "text_key" : element["textKey"],
    #                     "accreditation_url" : element["url"],
    #                     "type" : element["type"],
    #                     "accreditation_description" : element["tooltip"],
    #                 }
    #                 agentaccreditation = {
    #                     "accreditation_id" : element["id"],
    #                     "agent_id" : agent_section.get("accountNumber"),
    #                     "have" : True
    #                 }
    #                 accreditations.append(accreditation)
    #                 agentaccreditations.append(agentaccreditation)

    #         if agent_section.get("missingAccreditations", []):
    #             for element in agent_section["missingAccreditations"]:
    #                 accreditation = {
    #                     "accreditation_id" : element["id"],
    #                     "label" : element["label"],
    #                     "text_key" : element["textKey"],
    #                     "accreditation_url" : element["url"],
    #                     "type" : element["type"],
    #                     "accreditation_description" : element["tooltip"],
    #                 }
    #                 agentaccreditation = {
    #                     "accreditation_id" : element["id"],
    #                     "agent_id" : agent_section.get("accountNumber"),
    #                     "have" : False
    #                 }
    #                 accreditations.append(accreditation)
    #                 agentaccreditations.append(agentaccreditation)

    #         one_to_one = {
    #             "normal" : {
    #                 "property_id" : property_id,
    #                 "display_address" : property_section.get("displayAddress"),
    #                 "unit_number" : property_section.get("unitNumber"),
    #                 "site_number" : property_section.get("siteNumber"),
    #                 "building_name" : property_section.get("buildingName"),
    #                 "house_number" : property_section.get("house_number"),
    #                 "street" : property_section.get("street"),
    #                 "town" : property_section.get("town"),
    #                 "region" : property_section.get("region"),
    #                 "postcode" : property_section.get("postcode"),
    #                 "country" : property_section.get("coutry", {}).get("name"),
    #                 "lat" : property_section.get("coordinate", {}).get("latitude"),
    #                 "long" : property_section.get("coordinate", {}).get("longitude"),
    #                 "coordinate_accuracy" : property_section.get("coordinage", {}).get("accuracy"),
    #                 "bedrooms" : property_section.get("numBedrooms"),
    #                 "bathrooms" : property_section.get("numBathrooms"),
    #                 "receptions" : property_section.get("numReceptions"),
    #                 "property_type" : property_section.get("type"),
    #                 "comm_res" : property_section.get("propertyType", {}).get("key"),
    #                 "sale_type" : property_section.get("saleType", {}).get("key"),
    #                 "coming_soon" : property_section.get("comingSoon"),
    #                 "property_style" : style_section.get("text"),
    #                 "furnished" : property_section.get("furnishedType"),
    #                 "date_viewable_from" : property_section.get("dateViewableFrom"),
    #                 "date_available_from" : property_section.get("dateAvailableFrom"),
    #                 "date_added" : property_section.get("activationTime"), 
    #                 "continuousRelisting" : property_section.get("contiuousRelisting"),
    #                 "price_tracker" : property_section.get("priceTrackerAvailable"),
    #                 "total_images" : len(image_section),
    #                 "total_videos" : len(video_section),
    #                 "tagline" : property_section.get("tagline"),
    #                 "brief_text" : property_section.get("briefText"),
    #                 "description" : property_section.get("description"),
    #                 "show_home_opening_time" : property_section.get("showHomeOpeningTime"),
    #                 "short_listed" : property_section.get("shortlisted"),
    #                 "featurable" : property_section.get("featurable"),
    #                 "premium_listing" : property_section.get("premiumListing"),
    #                 "premier_available" : property_section.get("premiuerAvailable"),
    #                 "co_ownership" : property_section.get("coOwnershipEligible"),
    #                 "broadband_estimated" : property_section.get("estimated"),
    #                 "broadband_max_down" : property_section.get("maxPredictedDown"),
    #                 "broadband_max_up" : property_section.get("maxPredictedUp"),
    #                 "broadband_max_bb_down" : property_section.get("maxBbPredictedDown"),
    #                 "broadband_max_bb_up" : property_section.get("maxBbPredictedUp"),
    #                 "broadband_max_sfbb_down" : property_section.get("maxSfbbPredictedDown"),
    #                 "broadband_max_sfbb_up" : property_section.get("maxSfbbPredictedUp"),
    #                 "broadband_max_ufbb_down" : property_section.get("maxUfbbPredictedDown"),
    #                 "broadband_max_ufbb_up" : property_section.get("maxUfbbPredictedUp"),
    #             },
    #             "foreign_key" : {
    #                 "account_number" : agent_section.get("accountNumber")
    #             }
    #         }
    #         one_to_ones.append(one_to_one)

    #         for item in listing_history_section:
    #             listing_history = {
    #                 "data" : {
    #                     "price_qualifier" : item["pricePrefix"],
    #                     "price_suffix" : item["priceSuffix"],
    #                     "previous_status" : item["previousStatus"],
    #                     "price_difference" : item["difference"], 
    #                     "previous_price" : item["previousPrice"],
    #                     "previous_published" : item["previousPublished"],
    #                     "status_change" : item["statusChange"] if len(listing_history_section) > 1 else False,
    #                     "price_change" : item["priceChange"],
    #                     "published_changed" : item["publishedChange"],
    #                     "status" : item["status"].get("text"),
    #                     "price" : item["price"],
    #                     "update_date" : item["timeModified"],
    #                 },
    #                 "id" : {
    #                     "property_id" : property_id,
    #                 }
    #             }
                
    #             listing_histories.append(listing_history)

    #         for item in key_feature_section:
    #             key_feature = {
    #                 "data" : {
    #                     "feature_name" : item.get("name"),
    #                     "feature_text" : item.get("text")
    #                 },
    #                 "id" : {
    #                     "property_id" : property_id
    #                 }
    #             }

    #             key_features.append(key_feature)

    #         for item in view_section:
    #             view = {
    #                 "data" : {
    #                     "view_date" : element.get("date"),
    #                     "unique_views" : element.get("totalUniqueViews"),
    #                     "total_views" : element.get("totalViews"),
    #                     "featured" : element.get("featured"),
    #                     "bumped" : element.get("bumped"),
    #                     "published" : element.get("published"),
    #                 },
    #                 "id" : {
    #                     "property_id" : property_id,
    #                 }   
    #             }
    #             views.append(view)
        
    #     Property.objects.bulk_create(
    #         [Property(property_id=property_id)for property_id in properties]
    #     )

    #     Accreditation.objects.bulk_create(
    #         [Accreditation(
    #             **accreditation
    #         )for accreditation in accreditations],
    #         ignore_conflicts=True
    #     )

    #     EstateAgent.objects.bulk_create(
    #         [EstateAgent(
    #             **agent
    #         )for agent in agents],
    #         ignore_conflicts=True
    #     )

    #     AgentAccreditation.objects.bulk_create(
    #         [AgentAccreditation(
    #             accreditation_id = Accreditation.objects.get(accreditation_id = agentaccreditation.get("accreditation_id")),
    #             agent_id = EstateAgent.objects.get(account_number = agentaccreditation.get("agent_id")),
    #             have = agentaccreditation.get("have"),
    #         )for agentaccreditation in agentaccreditations],
    #         ignore_conflicts=True
    #     )

    #     PropertyData.objects.bulk_create(
    #         [PropertyData(
    #             agent = EstateAgent.objects.get(**one_to_one["foreign_key"]),
    #             **one_to_one["normal"],
    #         )for one_to_one in one_to_ones],
    #         ignore_conflicts=True
    #     )

    #     ListingHistory.objects.bulk_create(
    #         [ListingHistory(
    #             property_id = Property.objects.get(**listing_element["id"]),
    #             **listing_element["data"]
    #         )for listing_element in listing_histories]
    #     )

    #     KeyFeature.objects.bulk_create(
    #         [KeyFeature(
    #             property_id = Property.objects.get(**key_feature_element["id"]),
    #             **key_feature_element["data"]
    #         )for key_feature_element in key_features]
    #     )

    #     Views.objects.bulk_create(
    #         [Views(
    #             property_id = Property.objects.get(**view_element["id"]),
    #             **view_element["data"]
    #         )for view_element in views]
    #     )