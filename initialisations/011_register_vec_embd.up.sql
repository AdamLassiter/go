insert into temp.lembed_models (name, model)
select 'default', lembed_model_from_file('models/all-MiniLM-L6-v2.q8_0.gguf');
