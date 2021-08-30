package hello.hellospring;

import hello.hellospring.aop.TimeTraceAop;
import hello.hellospring.repository.*;
import hello.hellospring.service.MemberService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.jdbc.datasource.DataSourceUtils;

import javax.persistence.EntityManager;
import javax.sql.DataSource;

@Configuration
public class SpringConfig {

    //private DataSource dataSource;
    //@Autowired
    //public SpringConfig(DataSource dataSource) {
    //    this.dataSource = dataSource;
    //}

    //private EntityManager em;
    //@Autowired
    //public SpringConfig(EntityManager em) {
    //    this.em = em;
    //}

    private final MemberRepository memberRepository;

    @Autowired
    public SpringConfig(MemberRepository memberRepository) {
        this.memberRepository = memberRepository;
    }

    @Bean
    public MemberService memberService() {
        //return new MemberService(memberRepository());
        return new MemberService(memberRepository);
    }

    //@Bean
    //public MemberRepository memberRepository() {
        //return new MemoryMemberRepository();
        //return new JdbcMemberRepository(dataSource);
        //return new JdbcTemplateMemberRepository(dataSource);
        //return new JpaMemberRepository(em);
    //}

    // Spring Bean에 지정하면, 얘는 AOP를 걸어서 쓰는구나를 금방 인지할 수 있어서 좋음. 여기서는 그냥 컴포넌트 스캔을 썼음.
    //@Bean
    //public TimeTraceAop timeTraceAop() {
    //    return new TimeTraceAop();
    //}
}
