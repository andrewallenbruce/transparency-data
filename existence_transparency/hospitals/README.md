# Hospital Existence Transparency

The file `hospitals.csv` contains metadata related to the existence of known U.S. hospitals. 

It is a flat file with the following schema:

<!-- TODO: Store schema definitions as json. Add rust/c scripts to run as pre-commit hooks
that auto-update all documentation, csv headers, and validation scripts based on schema json -->

| Name | Description | Type | Definition | Required | Example Value |
| ----- | ---- | ---- | ---------- | -------- | --------- |
| **street_address** | Street address of hospital. | String | Street address of hospital | No | 1108 ross clark circle |
| **phone_number** | Phone number of hospital. | String | Phone number of hospital, with area code, without dashes or spaces | No | 3347938701 |
| **zip_code** | Zip code of hospital | String | Zip code of hospital. Five or Nine digit. | No | 36301 |
| **has_501c3** | Does the hospital have 501(c)(3) status as a charitable hospital. | Enum("true", "false") | Describes whether the organization is or is not a charitable hospital with 501(c)(3), in the sense dictated by the IRS. See [this](https://www.irs.gov/charities-non-profits/charitable-hospitals-general-requirements-for-tax-exemption-under-section-501c3) for more info. Note that this is not asking whether or not the organization is merely a nonprofit corporation as determined by relevant state law. | No |  |
| **fips_county_code** | Federal Information Processing System County Code | String | Three digit [FIPS county code](https://en.wikipedia.org/wiki/FIPS_county_code). | No | 069 |
| **bed_count** | Number of beds in facility. | Int | Number of beds in facility, if known. | No | 420 |
| **city** | Number of beds in facility. | Int | Number of beds in facility, if known. | No | dothan |
| **state_or_region** | State or Region | Enum(ABBREV) (see below) | State or region in which hospital is located. | Yes | AL |
| **name_common** | Common name of hospital. | String | The common name of the hospital | Yes | southeast health medical center|
| **name_legal** | Legal name of hospital. | String | The legal name of the hospital | No | |
| **ccn** | CMS certification number | String | A six to ten digit certfication string (can lead with 0s) given to a provider certified to bill medicare or medicaid. See [this](https://www.cms.gov/medicare/provider-enrollment-and-certification/surveycertificationgeninfo/downloads/survey-and-cert-letter-16-09.pdf) for more info on CCNs and the meaning of their digits. | No | 010001 |
| **facility_type** | Type of facility. | Enum("Short Term", "Critical Access Hospitals", "Long Term", "Rehabilitation", "Childrens Hospitals", "Psychiatric", "Transplant Hospitals", "Religious Non-Medical Health Care Institutions", "Medicaid Only Children's Psychiatric", "other") | The type of facility, if known or applicable. | No | Short Term |
| **medicare_medicaid_eligible** | Whether or not the hospital is eligible to bill medicare and/or medicaid | Enum("true", "false") | The name of a particular physical facility, if applicable | No | true |
| **lat** | Latitude | float | Precise latitude of facility. | No |  |
| **lon** | Longitude| float | Precise longitude of facility. | No | |
| **ein** | Federal employer Identification Number | String | The IRS employer identification number for the company. | No | |


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