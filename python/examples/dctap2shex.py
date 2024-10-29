from pyrudof import Rudof, RudofConfig, ShExFormat, ShExFormatter, DCTapFormat



rudof = Rudof(RudofConfig())
rudof.read_dctap_str("""shapeId,shapeLabel,propertyId,Mandatory,Repeatable,valueDatatype,valueShape
Person,Shape or person,name,true,false,xsd:string, 
,,birthdate,false,false,xsd:date""")

rudof.dctap2shex()
result = rudof.serialize_shex(ShExFormat.shexc(), ShExFormatter())
print(f"Result: {result}")
