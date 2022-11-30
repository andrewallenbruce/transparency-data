# Hospital Existence Transparency

The file `hospitals.csv` contains metadata related to the existence of known U.S. hospitals. 

It is a flat file with the following schema:

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **ccn** | CMS certification number | String | A six to ten digit certfication string (can lead with 0s) given to a provider certified to bill medicare or medicaid. See [this](https://www.cms.gov/medicare/provider-enrollment-and-certification/surveycertificationgeninfo/downloads/survey-and-cert-letter-16-09.pdf) for more info on CCNs and the meaning of their digits. | No | 010001 |
| **hospital_name_common** | Common name of hospital. | String | The common name of the hospital | No | |
| **hospital_name_legal** | Legal name of hospital. | String | The legal name of the hospital | No | |
| **facility_name** | Name of facility, if applicable. | String | The name of a particular physical facility, if applicable | No | |
| **state_or_region** | State or Region | Enum(ABBREV) (see below) | State or region in which hospital is located. | No | PA |
| **lat** | Latitude | float | Latitude of facility. | No |  |
| **lon** | Longitude| float | Longitude of facility. | No | |

TODO: Finish documenting schema.

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