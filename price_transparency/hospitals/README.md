# Hospital Price Transparency

The file `machine_readable_links.csv` provides links to known MRFs for hospital price transparency.

It is a flat file with the following schema:

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **ccn** | CMS certification number | String | A six to ten digit certfication string (can lead with 0s) given to a provider certified to bill medicare or medicaid. See [this](https://www.cms.gov/medicare/provider-enrollment-and-certification/surveycertificationgeninfo/downloads/survey-and-cert-letter-16-09.pdf) for more info on CCNs and the meaning of their digits. | No | 010001 |
| **reporting_entity_name_legal** | Entity Name (legal) | String | The legal name of the entity publishing the machine-readable file, i.e. the hospital's legal name. | No | childrens hospital of philadelphia foundation |
| **reporting_entity_name_common** | Entity Name (common) | String | The common name of the entity publishing the machine-readable file, i.e. the hospital's common name. | Yes | The Children's Hospital of Philadelphia |
| **reporting_entity_type** | Entity Type | Enum("hospital", "other") | The type of entity that is publishing the machine-readable file. | No | hospital |
| **machine_readable_url** | Machine Readable URL | String | A (purported) url for the machine readable file resource. | No |  	https://media.chop.edu/data/files/finance/23-1352166_Children%E2%80%99sHospitalofPhiladelphia_standardcharges.csv |
| **machine_readable_url_status** | Machine Readable URL Status | Enum("up", "down", "corrupt") | A status code for the purported url for the machine readable file resource. | Yes | up| 
| **machine_readable_page** | Consumer Page URL | String | URL for an official consumer facing page containing a link to the MRF, maintained by the reporting entity. | No | https://www.chop.edu/centers-programs/billing-and-insurance/understanding-hospital-charges |
| **supplemental_url** | Supplemental URL | String | A url for any supplemental information pertaining to the particular transparency MRF in question. | No |  |
| **file_name** | File Name | String | Default name of file served. | No | |
| **file_format** | File Format | Enum("csv", "json", "xml", "xlsx", "zip/csv", "zip/json", "zip/xml", "zip/xlsx", "other") | Format of the file. | Yes | csv |
| **meets_standard** | Meets Standard | Enum("true", "false") | Whether the MRF name and format meet the required standard. | No | true |
| **standard_issue** | Standard Issue | String | If standard is not met, a description of the discrepancy. | No |  |
| **state_or_region** | State or Region | Enum(ABBREV) (see below) | State or region in which legal reporting entity is incorporated. | No | PA |
| **last_updated_date** | Last Updated Date | String | The date in which the MRF was last updated according to the organization. Date must be in an ISO 8601 format (i.e. YYYY-MM-DD). | No | 2022-04-14 |
| **entry_date** | Entry Date | String | The date in which **this** record (not the MRF) was last updated/entered into the system. Date must be in an ISO 8601 format (i.e. YYYY-MM-DD). | Yes | 2022-04-14 |
| **notes** | Notes | String | Miscellaneous notes for each entry. | No | Notes for row |


**Note:**

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

While the `machine_readable_url` entry is not strictly required, _if_ it is populated it should be a URI for the actual MRF resource in accordance with the final rule.
It should not, for example, be the URI for a page with a button that allows one to download the MRF (such a page should be entered in the `machine_readable_page` field,
because such a page is not an MRF as the term is defined in the final rule). By enforcing this usage, we can ensure that the actual MRF data for the subset of rows for which a `machine_readable_url` is entered will be easy to pull down using only this dataset, without having to do any sort of additional crawling, etc.