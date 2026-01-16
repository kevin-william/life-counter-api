using AutoMapper;
using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Mappings;

public class MappingProfile : Profile
{
    public MappingProfile()
    {
        // Contador mappings
        CreateMap<Contador, ContadorDto>();
        CreateMap<ContadorCreateDto, Contador>();
        CreateMap<ContadorUpdateDto, Contador>()
            .ForAllMembers(opts => opts.Condition((src, dest, srcMember) => srcMember != null));
        
        // Utilizador mappings
        CreateMap<Utilizador, UtilizadorDto>();
        CreateMap<UtilizadorCreateDto, Utilizador>();
        CreateMap<UtilizadorUpdateDto, Utilizador>()
            .ForAllMembers(opts => opts.Condition((src, dest, srcMember) => srcMember != null));
        
        // Contador_Utilizador mappings
        CreateMap<Contador_Utilizador, Contador_UtilizadorDto>();
        CreateMap<Contador_UtilizadorCreateDto, Contador_Utilizador>();
        CreateMap<Contador_UtilizadorUpdateDto, Contador_Utilizador>()
            .ForAllMembers(opts => opts.Condition((src, dest, srcMember) => srcMember != null));
    }
}
