-- pragma load_extension = 1;
-- .load extensions/vec0;
-- .load extensions/lembed0;

insert into temp.lembed_models (name, model)
select 'minilm', lembed_model_from_file('models/all-MiniLM-L6-v2.q8_0.gguf');
