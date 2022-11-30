# Insurer Existence Transparency

This data collects metadata related to U.S. health insurers and the plans they sell. The data is broken down into the following three categories:

1.  Insurers

    The file [insurers.csv](./insurers.csv) contains metadata related to **insurance companies**.
2.  Issuers

    The file [issuers.csv](./issuers.csv) contains metadata related to insurance health plan **issuers**. 

3.  Plans


    The file [plans.csv](./plans.csv) contains metadata related to insurance health **plans**. 


We provide some more details on these three files below.

<details>
<summary>Insurers </summary>

**Insurance companies** are legal entities registered with federal and state governments that sell insurance. Each such entity can sell insurance
plans in one or many states.

This file is a flat file with the following schema:

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **homepage_url** | URL | String | a url for the insurers homepage on the web. | No | www.cignahealthspring.com |
| **state_or_regions** | State or Regions | Enum(ABBREV) (see below) | State or regions in which legal reporting entity sells insurance. Comma separated list. | Yes | DC,MD |
| **naic_company_code** | NAIC insurance company identifier | String | The [NAIC](https://content.naic.org/) insurance company level identifier. | No | 10095 | 
| **insurer_name_legal** | Legal name of insurance company. | String | The legal name of the issuer entity. | Yes | bravo health mid atlantic inc |
| **org_street_address** | Street Address | String | Street address for organizational mailing address of company. | No | 500 GREAT CIRCLE ROAD |
| **org_city** | City | String | City for organizational mailing address of company. | No | nashville |
| **org_state_or_region** | State or Region | String | State or Region code for organizational mailing address of company. | No | TN |
| **org_zip_code** | Zip code | String | Zip code for organizational mailing address of company. | No | 21224-5238 |
| **org_phone** | Phone number | String | Phone number for organizational mailing address of company. | No | 800-235-9188 |
| **ein** | Federal employer Identification Number | String | The IRS employer identification number for the company. | No | |

</details>

<details>
<summary>Issuers </summary>

To [quote](https://www.cms.gov/CCIIO/Resources/Files/faq_plan_finder_data_entry) CMS,

> **Issuers** represent the organization within an insurance company that is responsible for insurance offerings within a given state. 

Any health insurance company will have one issuer per state in which they are licensed to do business. 
Currently, this data is focused on Qualified Health Plans (QHP) as the term is [defined](https://www.cms.gov/CCIIO/Programs-and-Initiatives/Health-Insurance-Marketplaces/qhp) by CMS. This is merely
a matter of convenience at the moment, since such plans are easier to track via e.g. HIOS identifers. We ultimately want to track all U.S. health plans, whether qualified or not.

This file is a flat file with the following schema:

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **hios_issuer_id** | Health Insurance Oversight System Issuer ID | String | Five-digit int/string (not sure if it can lead with 0s) that identifies the issuer organization as assigned by Health Insurance Oversight System (HIOS). | Yes | 91450 |
| **insurer_name_legal** | Legal name of insurance company. | String | The legal name of the issuer entity. | No | Health Net of Arizona, Inc. |
| **state_or_region** | State or Region | Enum(ABBREV) (see below) | State or region in which legal reporting entity is incorporated. | Yes | AZ |
| **serff_id** | SERFF issuer identifier | String | The [NAIC](https://content.naic.org/) SERFF defined issuer identifier. To [quote](https://www.cms.gov/CCIIO/Resources/Files/faq_plan_finder_data_entry) CMS, "NAIC maintains a reporting service called SERFF which is used by most states and required by 27 states to track submissions from insurance carriers to state DOI commissioners." | No | | 
| **naic_company_code** | NAIC insurance company identifier | String | The [NAIC](https://content.naic.org/) insurance company level identifier. | No | | 

</details>

<details>
<summary>Plans </summary>

Here plans refer to particular products sold by issuers that have a well-defined collection of benefits. Specifying a plan should uniquely
specify the coordination of benefits, formulary, issuer, etc. Pricing for the plan will however depend on details of the consumer, such as exact
geographic location, age, and other stats, so there are for example many premiums for  given plan.

This file is a flat file with the following schema:

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **hios_issuer_id** | Health Insurance Oversight System Issuer ID | String | Five-digit int/string (not sure if it can lead with 0s) that identifies the issuer organization as assigned by Health Insurance Oversight System (HIOS). | Yes | 91450 |
| **hios_rbis_plan_id** | Health Insurance Oversight System (HIOS) Rate and Benefits Information System ID for the Plan | String | Fourteen character string that identifies the plan as specified in RBIS, if applicable. | No |  |
| **hpid** | Health plan identifier assigned to the plan. | String | [Health plan identifier](https://www.cms.gov/Regulations-and-Guidance/Administrative-Simplification/Unique-Identifier/HPID). | No |  |
| **marketplace_type** | Marketplace type specifier | Enum("individual", "small group", "large group") | Enum specifying marketplace type on which plan is sold, if applicable. | No |  |
| **hios_product_id** | HIOS product id for the plan. | String | Ten character string that identifies the product in HIOS, if applicable. | No |  |
| **plan_type** | Type of plan. | Enum("hmo", "ppo", "epo", "pos", "dental", "other")| High level type of contract. | No |  |
| **sob_url** | Url for summary of benefits | String | URL for publicly hosted summary of benefits for plan. | No |  |
| **full_benefits_url** | Url for full benefits contract | String | URL for publicly hosted copy of full benefits booklet. | No |  |
| **plan_metal_level** | Metal level of plan | Enum("platinum", "gold", "silver", "bronze", "catastrophic") | Metal level of plan as designated by [healthcare.gov](https://www.healthcare.gov/choose-a-plan/plans-categories/)  | No |  |

</details>

**Note:**

<details>
<summary> Why not store this all in a sqlite database? </summary>
<!-- TODO: Maintain sqlite DBs that get bi-directionally auto-synced with exported csvs via CI jobs. This will improve
out of the box utility, while allowing accesible editing from all participants. -->
The files below ought to be thought of as tables in a relational schema, and probably belong most naturally in a relational database.
In fact, this is how [Persius](https://persius.org) uses such data behind the scenes in our free tools, and the data here was initially populated as an export
of a populated SQL DB. We maintain flat files here with foreign key constraints relaxed to mere implications just to facilitate accessible access (both read, and write) for those
who aren't familiar with SQL, but who want to use or contribute to this data. Keeping this data up to date is going to constitute persistent work given current reporting standards,
and we want to engender effective collective action.
</details>


The state_or_region entry, if entered, must be one of the following ABBREV standardized abbreviation strings:

| State or Region | ABBREV |
| ----------        | --------- |
| Alabama |AL |
|Alaska | AK |
|American Samoa | AS |
|Arizona | AZ |
|Arkansas | AR |
|California | CA |
|Colorado | CO |
|Connecticut |CT |
|Delaware | DE
|District of Columbia | DC|
|Florida | FL|
|Georgia | GA|
| Guam | GU |
|Hawaii | HI|
|Idaho | ID|
|Illinois | IL|
|Indiana | IN|
|Iowa | IA|
|Kansas | KS|
|Kentucky | KY|
|Louisiana | LA|
|Maine | ME|
|Maryland | MD|
|Massachusetts | MA|
|Michigan | MI|
|Minnesota | MN|
|Mississippi | MS|
|Missouri | MO|
|Montana | MT|
|Nebraska | NE|
|Nevada| NV|
|New Hampshire | NH|
|New Jersey | NJ|
|New Mexico | NM|
|New York | NY|
|North Carolina | NC|
|North Dakota | ND|
| Northern Mariana Islands | MP |
|Ohio | OH|
|Oklahoma | OK|
|Oregon | OR|
|Pennsylvania | PA|
| Puerto Rico | PR |
|Rhode Island | RI|
|South Carolina | SC|
|South Dakota | SD|
|Tennessee | TN|
|Texas | TX|
|Utah | UT|
|Vermont | VT|
| Virgin Islands | VI |
|Virginia | VA|
|Washington | WA|
| West Virginia | WV |
|Wisconsin | WI|
|Wyoming | WY|