# User stories (how different users interact with features to gain benefit): 
- **As an individual donor**, I want a straightforward way to donate my old furniture, allowing me to clear out space and give my items a second life
- **As an individual donor**, I want to specify the donation listing duration (start and end time) to fulfill my apartment move-out deadlines while avoiding the cost of moving it myself.  
- **As an individual donor**, I want to share detailed descriptions of my items so recipients know exactly what I'm offering and correctly evaluate if it fits their needs.
- **As an individual donor**, I want to communicate directly with the recipient to easily arrange/schedule a pickup time so it doesn't disrupt my day (donating without the hassle of coordination, phone calls, scheduling pickup, verifying decisions with the other party, etc).
- **As an individual donor**, I want to have a safe and easy way to share my contact details with recipients.
- **As an individual recipient**, I want to get an alert when my item is claimed, so I can arrange the pickup.
- **As an individual donor**, I want to search and join relevant organizations that align with my values to make a positive difference.
- **As an individual donor**, I want to find a convenient drop-off place to donate my stuff and continue with my day uninterrupted.
- **As a non-profit donor**, I want to coordinate drop-off/pickup timeframes for in-kind donations, so I can contribute effectively (fulfill mission) without incurring additional time or financial burdens.
- **As an organization recipient**, I want to easily find specific donation items that meet my needs and track my requests, so we can plan for item arrival and make the most out of donations received.
- **As a recipient** with a tight budget, I want to easily browse a variety of free listings for items in good condition, so that I can improve my quality of life without spending a lot of money.
- **As a recipient** who looks for free stuff, I want to browse by category and location so I can find things that are relevant to my needs.
- **As a recipient**, If I'm looking for specific items, I want to be able to set up alerts for when relavent items are posted.  

# Features / technical requirements (what capability the product has?): 
- create community and groups - explore community, join groups. 
  - community priority: prioritize giveaways first to close circles then expand to wider community circles. 
- offer donation items - list, browse, search & filter, categorize, location, save items;
  - description, photos, custom attributes table (AI-assistent filling), condition;
- request needs: wishlist of items, browse others needs, 
  - claim an offer: instant claim, claim after communication requiring approval.
  - automatic match requests to items
- transaction coordination pickup/dropoff: window with deadline, schedule, location, method (how to deliver); 
  - Allow organizations or individuals to collaborate with each other on arranging give away events. share schedule and responsibility. 
- communicate: message user (templated action-oriented messages); share contact info privately; join/follow groups for specific causes; status notifications. Alert for relavent items. 
- Reputation reviews: rate activity and interaction; verify recipient needs and prioritize individuals.

# ERD, schema & normalization: 
## idea description for database design

permit **owners** to post **items** with text & photos, and mark status as available/pending/claimed. **Customers**/beneficiaries can view **contact detail** and send **messages** to items' owners to coordinate pickup date & time **appointment**. 

## List of namespaces/aspects
- user: relationship between users and groups
- listing
- exchange: fulfilling request/order, coordination of exchange, logistics
- communicate

## List of enteties

1. listing.donation_offer
2. listing.donation_request
3. listing.category
    - product/goods/merchandise category classifying items (not following any classification standard/system) into groups based on function/purpose, context of usage, target audience, general broad characteristics defining intended primary use / form/material / principles of operation, etc, 
---
1.  exchange.transaction
    - records the transaction details between the users
2.  exchange.schedule
    - exchange opportunities as specific date-time or window timeframe (handling both flexible nad fixed times)
3.  exchange.track
    - updates on transaction, cancellation, timing change, activity etc.
---
1.  communicate.message
2.  communicate.event
3.  communicate.reputation
