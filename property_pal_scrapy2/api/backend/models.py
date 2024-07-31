from django.db import models

class Property(models.Model):
    property_id = models.CharField(max_length=20, primary_key=True)
    url = models.URLField(max_length=200)
    published = models.BooleanField(default=True)
    property_data = models.OneToOneField("PropertyData", on_delete=models.CASCADE, null=True)

class PropertyData(models.Model):
    property_id = models.TextField(primary_key=True)
    agent = models.ForeignKey("EstateAgent", on_delete=models.CASCADE, null=True)
    display_address = models.TextField(null=True)
    unit_number = models.TextField(null=True)
    site_number = models.TextField(null=True)
    building_name = models.TextField(null=True)
    house_number = models.TextField(null=True)
    street = models.TextField(null=True)
    town = models.TextField(null=True)
    region = models.TextField(null=True)
    postcode = models.TextField(null=True)
    country = models.TextField(null=True)
    lat = models.DecimalField(max_digits=50, decimal_places=10, null=True)
    long = models.DecimalField(max_digits=50, decimal_places=10, null=True)
    coordinate_accuracy = models.TextField(null=True)
    bedrooms = models.IntegerField(null=True)
    bathrooms = models.IntegerField(null=True)
    receptions = models.IntegerField(null=True)
    property_type = models.TextField(null=True)
    comm_res = models.TextField(null=True)
    sale_type = models.TextField(null=True)
    coming_soon = models.BooleanField(null=True)
    property_style = models.TextField(null=True)
    furnished = models.TextField(null=True)
    date_viewable_from = models.TextField(null=True)
    date_available_from = models.TextField(null=True)
    date_added = models.TextField(null=True)
    open_viewing = models.BooleanField(null=True)
    continuousRelisting = models.BooleanField(null=True)
    auction = models.BooleanField(null=True)
    price_tracker = models.BooleanField(null=True)
    total_images = models.IntegerField(null=True)
    total_videos = models.IntegerField(null=True)
    tagline = models.TextField(null=True)
    brief_text = models.TextField(null=True)
    description = models.TextField(null=True)
    show_home_opening_time = models.TextField(null=True)
    short_listed = models.BooleanField(null=True)
    featurable = models.BooleanField(null=True)
    premium_listing = models.BooleanField(null=True)
    premier_available = models.BooleanField(null=True)
    co_ownership = models.BooleanField(null=True)
    broadband_estimated = models.BooleanField(null=True)
    broadband_max_down = models.TextField(null=True)
    broadband_max_up = models.TextField(null=True)
    broadband_max_bb_down = models.TextField(null=True)
    broadband_max_bb_up = models.TextField(null=True)
    broadband_max_sfbb_down = models.TextField(null=True)
    broadband_max_sfbb_up = models.TextField(null=True)
    broadband_max_ufbb_down = models.TextField(null=True)
    broadband_max_ufbb_up = models.TextField(null=True)
    
class EstateAgent(models.Model):
    account_number = models.TextField(primary_key=True)
    agent_name = models.TextField(null=True)
    branch_name = models.TextField(null=True)
    display_address = models.TextField(null=True)
    agent_url = models.URLField(max_length=200, null=True)
    private_developer = models.BooleanField(null=True)
    developer_primary_contact = models.BooleanField(null=True)
    developer = models.BooleanField(null=True)
    agent_tier = models.TextField(null=True)
    agent_enhanced_branding = models.BooleanField(null=True)
    accreditations = models.ManyToManyField("Accreditation", through="AgentAccreditation", through_fields=("agent_id", "accreditation_id"))

class Accreditation(models.Model):
    accreditation_id = models.IntegerField(primary_key=True)
    label = models.TextField(null=True)
    text_key = models.TextField(null=True)
    accreditation_url = models.URLField(max_length=200, null=True)
    type = models.TextField(null=True)
    accreditation_description = models.TextField(null=True)

class AgentAccreditation(models.Model):
    agent_id = models.ForeignKey(EstateAgent, on_delete=models.CASCADE)
    accreditation_id = models.ForeignKey(Accreditation, on_delete=models.CASCADE)
    have = models.BooleanField()

    class Meta:
        unique_together = [["agent_id", "accreditation_id"]]

class ListingHistory(models.Model): 
    property_id = models.ForeignKey(Property, on_delete=models.CASCADE)
    price_qualifier = models.TextField(null=True)
    price_suffix = models.TextField(null=True)
    previous_status = models.TextField(null=True)
    price_difference = models.IntegerField(null=True)
    previous_price = models.IntegerField(null=True)
    previous_published = models.BooleanField(null=True)
    status_change = models.BooleanField(null=True)
    price_change = models.BooleanField(null=True)
    published_changed = models.BooleanField(null=True)
    status = models.TextField(null=True)
    price = models.IntegerField(null=True)
    update_date = models.TextField(null=True)

    class Meta:
        unique_together = [["property_id", "update_date"]]

class KeyFeature(models.Model):
    property_id = models.ForeignKey(Property, on_delete=models.CASCADE, null=True)
    feature_name = models.TextField(null=True)
    feature_text = models.TextField(null=True)
    
    class Meta:
        unique_together = [["property_id", "feature_name", "feature_text"]]

class Views(models.Model):
    property_id = models.ForeignKey(Property, on_delete=models.CASCADE, null=True)
    view_date = models.TextField(null=True)
    unique_views = models.IntegerField(null=True)
    total_views = models.IntegerField(null=True)
    featured = models.BooleanField(null=True)
    bumped = models.BooleanField(null=True)
    published = models.BooleanField(null=True)

    class Meta:
        unique_together = [["property_id", "view_date"]]